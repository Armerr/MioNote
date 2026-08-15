use regex::Regex;

use crate::models::SearchResult;

pub fn extract_tags(content: &str) -> Vec<String> {
    let mut tags = std::collections::BTreeSet::new();
    for word in content.split_whitespace() {
        let candidate = word.trim_matches(|character: char| {
            !character.is_ascii_alphanumeric()
                && character != '#'
                && character != '_'
                && character != '-'
        });
        if let Some(tag) = candidate.strip_prefix('#') {
            if !tag.is_empty()
                && tag
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
            {
                tags.insert(tag.to_ascii_lowercase());
            }
        }
    }
    tags.into_iter().collect()
}

pub fn search_content(
    title: &str,
    content: &str,
    last_modified: f64,
    term: &str,
) -> Option<SearchResult> {
    let term = term.trim();
    if term.is_empty() {
        return None;
    }
    let tags = extract_tags(content);
    let mut text_terms = Vec::new();
    let mut tag_terms = Vec::new();
    for raw in term.split_whitespace() {
        let token = raw.trim_matches('"');
        if let Some(tag) = token.strip_prefix('#') {
            tag_terms.push(tag.to_ascii_lowercase());
        } else if let Some(value) = token.strip_prefix("title:") {
            text_terms.push(("title", value.to_ascii_lowercase()));
        } else if let Some(value) = token.strip_prefix("content:") {
            text_terms.push(("content", value.to_ascii_lowercase()));
        } else if token != "*" {
            text_terms.push(("any", token.to_ascii_lowercase()));
        }
    }
    let title_lower = title.to_ascii_lowercase();
    let content_lower = content.to_ascii_lowercase();
    let tags_match = tag_terms
        .iter()
        .all(|query| tags.iter().any(|tag| tag == query));
    let text_match = text_terms.iter().all(|(field, query)| match *field {
        "title" => title_lower.contains(query),
        "content" => content_lower.contains(query),
        _ => {
            title_lower.contains(query)
                || content_lower.contains(query)
                || tags.iter().any(|tag| tag.contains(query))
        }
    });
    if term != "*" && (!tags_match || !text_match) {
        return None;
    }

    let mut matched_terms: Vec<String> = text_terms
        .iter()
        .map(|(_, value)| value.clone())
        .chain(tag_terms.iter().cloned())
        .collect();
    matched_terms.sort();
    matched_terms.dedup();
    let score = matched_terms
        .iter()
        .map(|query| title_lower.matches(query).count() + content_lower.matches(query).count())
        .sum::<usize>() as f32;
    let title_highlights = if !matched_terms.is_empty() && title_lower.contains(&matched_terms[0]) {
        Some(highlight(title, &matched_terms))
    } else {
        None
    };
    let content_highlights =
        if !matched_terms.is_empty() && content_lower.contains(&matched_terms[0]) {
            let preview = content
                .lines()
                .find(|line| line.to_ascii_lowercase().contains(&matched_terms[0]))
                .unwrap_or(content);
            Some(highlight(preview, &matched_terms))
        } else {
            None
        };
    let tag_matches = if tag_terms.is_empty() {
        None
    } else {
        Some(
            tags.into_iter()
                .filter(|tag| tag_terms.iter().any(|query| tag == query))
                .collect(),
        )
    };
    Some(SearchResult {
        title: title.to_string(),
        preview: content_preview(content),
        last_modified,
        score: Some(score),
        title_highlights,
        content_highlights,
        tag_matches,
    })
}

fn content_preview(content: &str) -> Option<String> {
    let preview = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(strip_preview_markup)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(140)
        .collect::<String>();
    (!preview.is_empty()).then_some(preview)
}

fn strip_preview_markup(line: &str) -> String {
    let without_html = line
        .chars()
        .scan(false, |inside_tag, character| match character {
            '<' => {
                *inside_tag = true;
                Some(None)
            }
            '>' => {
                *inside_tag = false;
                Some(None)
            }
            _ if *inside_tag => Some(None),
            _ => Some(Some(character)),
        })
        .flatten()
        .collect::<String>();

    without_html
        .replace("**", "")
        .replace("__", "")
        .replace("~~", "")
        .replace('`', "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn highlight(value: &str, terms: &[String]) -> String {
    let mut result = html_escape(value);
    for term in terms {
        if term.is_empty() {
            continue;
        }
        let pattern = match Regex::new(&format!("(?i){}", regex::escape(term))) {
            Ok(pattern) => pattern,
            Err(_) => continue,
        };
        result = pattern
            .replace_all(&result, |captures: &regex::Captures| {
                format!("<span class=\"match\">{}</span>", &captures[0])
            })
            .to_string();
    }
    result
}

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::{extract_tags, search_content};

    #[test]
    fn extracts_unique_lowercase_tags() {
        assert_eq!(
            extract_tags("#Rust #rust #note-taking punctuation:#ignored"),
            vec!["note-taking".to_string(), "rust".to_string()]
        );
    }

    #[test]
    fn returns_escaped_highlights_for_tag_search() {
        let result = search_content(
            "Rust <Guide>",
            "#rust\nRich text writes **Markdown**.",
            1.0,
            "#rust",
        )
        .expect("tag should match");
        assert_eq!(result.tag_matches, Some(vec!["rust".to_string()]));
        assert_eq!(
            result.preview,
            Some("Rich text writes Markdown.".to_string())
        );
        assert_eq!(
            result.content_highlights,
            Some("#<span class=\"match\">rust</span>".to_string())
        );
        assert_eq!(
            result.title_highlights,
            Some("<span class=\"match\">Rust</span> &lt;Guide&gt;".to_string())
        );
    }
}
