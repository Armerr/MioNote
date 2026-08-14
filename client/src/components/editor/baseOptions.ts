import codeSyntaxHighlight from "@toast-ui/editor-plugin-code-syntax-highlight/dist/toastui-editor-plugin-code-syntax-highlight-all.js";
import router from "../../router";

const customHTMLRenderer = {
  // Add id attribute to headings
  heading(node, { entering, getChildrenText, origin }) {
    const original = origin();
    if (entering) {
      original.attributes = {
        id: getChildrenText(node)
          .toLowerCase()
          .replace(/[^\p{L}\p{N}\s-]*/gu, "")
          .trim()
          .replace(/\s/g, "-"),
      };
    }
    return original;
  },
  // Convert relative hash links to absolute links
  link(_, { entering, origin }) {
    const original = origin();
    if (entering) {
      const href = original.attributes.href;
      if (href.startsWith("#")) {
        const targetRoute = {
          ...router.currentRoute.value,
          hash: href,
        };
        original.attributes.href = router.resolve(targetRoute).href;
      }
    }
    return original;
  },
  // Preserve the inline formatting emitted by the Vue-owned toolbar in both
  // Markdown and WYSIWYG mode. Toast UI otherwise treats these elements as
  // unknown HTML and drops their attributes during its model conversion.
  htmlInline: {
    span(node, { entering }) {
      return {
        type: entering ? "openTag" : "closeTag",
        tagName: "span",
        attributes: entering ? node.attrs : undefined,
      };
    },
    mark(node, { entering }) {
      return {
        type: entering ? "openTag" : "closeTag",
        tagName: "mark",
        attributes: entering ? node.attrs : undefined,
      };
    },
    u(node, { entering }) {
      return {
        type: entering ? "openTag" : "closeTag",
        tagName: "u",
        attributes: entering ? node.attrs : undefined,
      };
    },
  },
  htmlBlock: {
    video(node, { entering }) {
      return {
        type: entering ? "openTag" : "closeTag",
        tagName: "video",
        attributes: entering ? node.attrs : undefined,
      };
    },
  },
};

const baseOptions = {
  height: "100%",
  minHeight: "0px",
  plugins: [codeSyntaxHighlight],
  customHTMLRenderer: customHTMLRenderer,
  toolbarItems: [],
  hideModeSwitch: true,
  usageStatistics: false,
};

export default baseOptions;
