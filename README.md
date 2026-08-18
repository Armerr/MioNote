<p align="center">
  <img src="client/public/favicon.svg" width="96" alt="MioNote" />
</p>

<h1 align="center">MioNote</h1>

<p align="center">一款以 Markdown 为核心的自托管笔记工作空间</p>

<p align="center">
  <a href="README.en.md">English</a>
  ·
  <a href="LICENSE">MIT License</a>
</p>

<p align="center"><code>v0.1.2</code> · 默认中文 · 支持 English · Docker 就绪</p>

MioNote 面向个人和小型团队，提供多用户账户、Markdown 笔记、富文本编辑、全文搜索、标签、附件和 Wiki 链接。数据以普通 Markdown 文件保存，数据库只负责账户信息，迁移和备份保持简单透明。

## 目录

- [核心能力](#核心能力)
- [快速开始](#快速开始)
- [Docker Compose](#docker-compose)
- [配置](#配置)
- [数据与备份](#数据与备份)
- [子路径部署](#子路径部署)
- [本地开发](#本地开发)
- [Agent Skill](#agent-skill)
- [API 文档](#api-文档)

## 核心能力

| 领域 | 能力 |
| --- | --- |
| 账户 | 注册、登录、JWT 会话；密码使用 Argon2 哈希保存 |
| 笔记 | Markdown 源码与所见即所得编辑、无标题笔记、标签、Wiki 链接 |
| 搜索 | 按标题、正文和 `#标签` 全文检索，并支持相关度、标题和修改时间排序 |
| 文件 | 附件上传；笔记、附件和数据库均可通过数据目录迁移 |
| 体验 | 浅色/深色主题、中文/English 切换、桌面端与平板响应式工作区 |
| 部署 | Docker、Docker Compose，以及反向代理子路径部署 |

## 快速开始

推荐使用预构建镜像。下面的示例将数据持久化到当前目录的 `data/`，并在首次访问时创建账户。

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

打开 <http://localhost:4233>，在登录页创建第一个账户。注册默认开启；如需关闭公开注册，增加：

```shell
-e MIONOTE_REGISTRATION_OPEN=false
```

## Docker Compose

将以下内容保存为 `compose.yaml`：

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

启动服务：

```shell
docker compose up -d
```

## 配置

| 变量 | 默认值 | 说明 |
| --- | --- | --- |
| `MIONOTE_PATH` | 必填；容器内为 `/data` | 笔记、附件与 SQLite 数据库的根目录。 |
| `MIONOTE_SECRET_KEY` | 必填 | 用于签发 JWT 的随机密钥，应使用足够长且不可预测的值。 |
| `MIONOTE_HOST` | `0.0.0.0` | 服务监听地址。 |
| `MIONOTE_PORT` | `4233` | 服务监听端口。 |
| `MIONOTE_REGISTRATION_OPEN` | `true` | 是否允许创建新账户。 |
| `MIONOTE_SESSION_EXPIRY_DAYS` | `30` | 登录会话有效期，单位为天。 |
| `MIONOTE_PATH_PREFIX` | 空 | 反向代理子路径，例如 `/notes`。必须以 `/` 开头，不能以 `/` 结尾。 |

服务日志默认输出 `INFO` 级别的启动、请求结果和写入事件到标准输出。可以通过 `RUST_LOG` 调整级别，例如 `RUST_LOG=debug`。日志不会记录认证凭据、令牌、笔记内容或动态路由参数。

## 数据与备份

数据目录结构如下：

```text
<MIONOTE_PATH>/mionote.db
<MIONOTE_PATH>/users/<user-id>/<笔记标题>.md
<MIONOTE_PATH>/users/<user-id>/attachments/<文件名>
```

SQLite 数据库始终位于 `MIONOTE_PATH/mionote.db`，不支持配置到其他目录。备份 `MIONOTE_PATH` 下的整个目录即可同时备份账户、笔记和附件；恢复前请停止服务，并确保容器运行用户拥有目录读写权限。

已有单用户目录中的根 Markdown 文件不会自动归属给注册账户，以避免意外暴露内容。迁移时，请将文件移动到目标账户目录。

## 子路径部署

当应用通过反向代理暴露在 `https://example.com/notes` 时，设置：

```shell
MIONOTE_PATH_PREFIX=/notes
```

反向代理需要将 `/notes` 及其全部子路径原样转发给 MioNote，不要在转发时剥离此前缀。

## 本地开发

安装前端依赖并构建生产静态文件：

```shell
npm ci
npm run build
```

启动后端：

```shell
MIONOTE_PATH=./data \
MIONOTE_SECRET_KEY=development-secret \
cargo run
```

如需前端热更新，在另一个终端启动 Vite 开发服务器：

```shell
MIONOTE_PATH=./data \
MIONOTE_SECRET_KEY=development-secret \
MIONOTE_PORT=4234 \
cargo run

MIONOTE_DEV_API_URL=http://127.0.0.1:4234 npm run dev
```

前端源码位于 `client/src/`，Rust 服务位于 `src/`。生产后端直接提供 `client/dist` 中的静态文件；Vite 默认监听 <http://127.0.0.1:5173/>。

## Agent Skill

MioNote 的自然语言记录能力作为 npm 项目 Skill 安装，不使用软链接。安装后重新打开 Agent 或开始新一轮对话：

```shell
npm install --save-dev github:Armerr/SKILL
```

项目根目录的 `AGENTS.md` 会指向 npm 包内的 `SKILL.md`。配置 MioNote 地址和认证信息后，可以先检查登录：

```shell
export MIONOTE_URL="http://127.0.0.1:4233"
export MIONOTE_ACCESS_TOKEN="..."
# 或使用 MIONOTE_USERNAME 和 MIONOTE_PASSWORD

npx mionote-capture-note --auth-check
```

访问令牌优先；用户名和密码只用于本次换取 JWT，不会写入 Skill、笔记或日志。Skill 默认直接归类并保存，只有语义不明确、疑似重复或无法定位更新/删除目标时才会询问。

## API 文档

服务运行后提供 API 索引和 OpenAPI 文档：

- <http://localhost:4233/docs>
- <http://localhost:4233/openapi.json>

主要接口包括：

| 方法 | 路径 | 用途 |
| --- | --- | --- |
| `POST` | `/api/token` | 使用用户名和密码获取 JWT |
| `POST` | `/api/register` | 注册账户（受 `MIONOTE_REGISTRATION_OPEN` 控制） |
| `GET` | `/api/users/me` | 获取当前账户 |
| `GET` | `/api/search` | 搜索当前账户的笔记 |
| `POST` | `/api/notes` | 创建笔记 |
| `GET/PATCH/DELETE` | `/api/notes/{title}` | 读取、更新或删除笔记 |
| `POST` | `/api/attachments` | 上传附件 |

除登录和注册接口外，请在请求头中携带 `Authorization: Bearer <token>`。使用 `MIONOTE_PATH_PREFIX` 时，请在接口地址前加上对应前缀。

## 测试

```shell
cargo test
```

## 许可证

[MIT License](LICENSE)
