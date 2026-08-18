<p align="center">
  <img src="client/public/favicon.svg" width="96" alt="MioNote" />
</p>

<h1 align="center">MioNote</h1>

<p align="center">A self-hosted note workspace built around Markdown</p>

<p align="center">
  <a href="README.md">简体中文</a>
  ·
  <a href="LICENSE">MIT License</a>
</p>

<p align="center"><code>v0.1.2</code> · Chinese by default · English supported · Docker ready</p>

MioNote is a self-hosted, multi-user note workspace for individuals and small teams. It combines Markdown storage with a rich-text editor, full-text search, tags, attachments, and Wiki links. Notes and attachments remain ordinary files; SQLite stores account data only, keeping migration and backup straightforward.

## Contents

- [Capabilities](#capabilities)
- [Quick Start](#quick-start)
- [Docker Compose](#docker-compose)
- [Configuration](#configuration)
- [Data and Backups](#data-and-backups)
- [Subpath Deployment](#subpath-deployment)
- [Local Development](#local-development)
- [Agent Skill](#agent-skill)
- [API Documentation](#api-documentation)

## Capabilities

| Area | Capabilities |
| --- | --- |
| Accounts | Registration, login, JWT sessions, and Argon2 password hashing |
| Notes | Markdown source and WYSIWYG editing, untitled notes, tags, and Wiki links |
| Search | Full-text search across titles, content, and `#tags`, with relevance, title, and modification-time sorting |
| Files | Attachment uploads; notes, attachments, and the database can be migrated together |
| Experience | Light/dark themes, Chinese/English switching, and responsive desktop/tablet workspaces |
| Deployment | Docker, Docker Compose, and reverse-proxy subpath deployment |

## Quick Start

Use the pre-built image for the fastest setup. This example persists all data in `./data` and lets you create the first account from the login page.

```shell
docker run -d \
  --name mionote \
  -e PUID=1000 \
  -e PGID=1000 \
  -e MIONOTE_SECRET_KEY="replace-with-a-long-random-secret" \
  -v "$(pwd)/data:/data" \
  -p 4233:4233 \
  armerr/mionote:0.1.2
```

Open <http://localhost:4233> and create the first account. Registration is enabled by default. To disable public registration, add:

```shell
-e MIONOTE_REGISTRATION_OPEN=false
```

## Docker Compose

Save the following as `compose.yaml`:

```yaml
services:
  mionote:
    image: armerr/mionote:0.1.2
    container_name: mionote
    environment:
      PUID: 1000
      PGID: 1000
      MIONOTE_SECRET_KEY: "replace-with-a-long-random-secret"
      # MIONOTE_REGISTRATION_OPEN: "false"
    volumes:
      - "./data:/data"
    ports:
      - "4233:4233"
    restart: unless-stopped
```

Start the service:

```shell
docker compose up -d
```

## Configuration

| Variable | Default | Description |
| --- | --- | --- |
| `MIONOTE_PATH` | Required; `/data` in the container | Root directory for notes, attachments, and SQLite. |
| `MIONOTE_SECRET_KEY` | Required | Random secret used to sign JWTs. Use a long, unpredictable value. |
| `MIONOTE_HOST` | `0.0.0.0` | Listen address. |
| `MIONOTE_PORT` | `4233` | Listen port. |
| `MIONOTE_REGISTRATION_OPEN` | `true` | Whether new accounts may be created. |
| `MIONOTE_SESSION_EXPIRY_DAYS` | `30` | Session lifetime in days. |
| `MIONOTE_PATH_PREFIX` | Empty | Reverse-proxy subpath, for example `/notes`. It must start with `/` and must not end with `/`. |

The service logs startup, request results, and write events at `INFO` by default. Set `RUST_LOG` to change the filter, for example `RUST_LOG=debug`. Logs do not include credentials, tokens, note content, or dynamic route parameters.

## Data and Backups

The data directory uses this layout:

```text
<MIONOTE_PATH>/mionote.db
<MIONOTE_PATH>/users/<user-id>/<note-title>.md
<MIONOTE_PATH>/users/<user-id>/attachments/<filename>
```

SQLite always lives at `MIONOTE_PATH/mionote.db` and cannot be configured elsewhere. Back up the entire `MIONOTE_PATH` directory to preserve accounts, notes, and attachments. Stop the service before restoring a backup and keep the data directory writable by the container user.

Markdown files placed at the root of an existing single-user data directory are not automatically assigned to an account, preventing accidental exposure. Move them into the target account directory during migration.

## Subpath Deployment

To expose MioNote through a reverse proxy at `https://example.com/notes`, set:

```shell
MIONOTE_PATH_PREFIX=/notes
```

The proxy must forward `/notes` and all of its child paths without stripping the prefix.

## Local Development

Install frontend dependencies and build the production assets:

```shell
npm ci
npm run build
```

Start the backend:

```shell
MIONOTE_PATH=./data \
MIONOTE_SECRET_KEY=development-secret \
cargo run
```

For frontend hot reload, start Vite in another terminal:

```shell
MIONOTE_PATH=./data \
MIONOTE_SECRET_KEY=development-secret \
MIONOTE_PORT=4234 \
cargo run

MIONOTE_DEV_API_URL=http://127.0.0.1:4234 npm run dev
```

Frontend sources are under `client/src/`; Rust sources are under `src/`. The production backend serves the built files in `client/dist`. Vite listens on <http://127.0.0.1:5173/> by default.

## Agent Skill

MioNote's natural-language capture capability is distributed as an npm project Skill, without symlinks. After installation, restart the Agent or start a new turn:

```shell
npm install --save-dev github:Armerr/SKILL
```

The project `AGENTS.md` points the Agent to the installed `SKILL.md`. Configure the MioNote URL and credentials, then verify authentication:

```shell
export MIONOTE_URL="http://127.0.0.1:4233"
export MIONOTE_ACCESS_TOKEN="..."
# Or use MIONOTE_USERNAME and MIONOTE_PASSWORD

npx mionote-capture-note --auth-check
```

Access tokens are preferred. Username and password are used only to obtain a JWT for the current invocation and are never written to the Skill, notes, or logs. The Skill saves directly by default and asks only when the meaning is unclear, a likely duplicate exists, or an update/delete target cannot be identified.

## API Documentation

When the service is running, API indexes are available at:

- <http://localhost:4233/docs>
- <http://localhost:4233/openapi.json>

Key endpoints:

| Method | Path | Purpose |
| --- | --- | --- |
| `POST` | `/api/token` | Exchange username and password for a JWT |
| `POST` | `/api/register` | Register an account (controlled by `MIONOTE_REGISTRATION_OPEN`) |
| `GET` | `/api/users/me` | Read the current account |
| `GET` | `/api/search` | Search the current account's notes |
| `POST` | `/api/notes` | Create a note |
| `GET/PATCH/DELETE` | `/api/notes/{title}` | Read, update, or delete a note |
| `POST` | `/api/attachments` | Upload an attachment |

All endpoints except login and registration require `Authorization: Bearer <token>`. Add `MIONOTE_PATH_PREFIX` to the beginning of each URL when a subpath is configured.

## Tests

```shell
cargo test
```

## License

[MIT License](LICENSE)
