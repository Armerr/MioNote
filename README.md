<p align="center">
  <img src="client/public/favicon.svg" width="120" alt="MioNote" />
</p>

# MioNote

MioNote 是一个自托管、多用户、以 Markdown 文件为核心的笔记应用。界面默认使用中文，也可切换 English；编辑器支持 Markdown 源码和所见即所得模式，所有笔记均以标准 Markdown 保存。

当前版本：`0.1.1`

## 功能

- 注册、登录与 JWT 会话，密码使用 Argon2 哈希保存。
- 每个账户的数据相互隔离，笔记和附件仍以普通文件保存，便于迁移和备份。
- Markdown 与所见即所得编辑、标签、全文检索和 Wiki 链接。
- 附件上传、浅色/深色主题、中文/English 界面和响应式工作区。
- 支持部署在反向代理的子路径下。

## 数据目录

账户信息保存在 SQLite，笔记与附件保留为文件：

```text
<MIONOTE_PATH>/mionote.db
<MIONOTE_PATH>/users/<user-id>/<笔记标题>.md
<MIONOTE_PATH>/users/<user-id>/attachments/<文件名>
```

SQLite 数据库始终存放在 `MIONOTE_PATH/mionote.db`，不支持设置到其他目录。

已有单用户目录中的根 Markdown 文件不会自动归属给注册账户，以免意外暴露内容。迁移时，请将文件移动到目标账户的目录中。

## Docker 部署

构建镜像：

```shell
docker build -t armerr/mionote:0.1.1 .
```

运行容器：

```shell
docker run -d \
  --name mionote \
  -e PUID=1000 \
  -e PGID=1000 \
  -e MIONOTE_SECRET_KEY="replace-with-a-long-random-secret" \
  -v "$(pwd)/data:/data" \
  -p 4233:4233 \
  armerr/mionote:0.1.1
```

访问 `http://localhost:4233`，在登录页创建第一个账户。注册默认开启；如需仅允许已有账户登录，设置 `MIONOTE_REGISTRATION_OPEN=false`。

## Docker Compose

```yaml
services:
  mionote:
    image: armerr/mionote:0.1.1
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

在包含该 Compose 文件的目录中执行：

```shell
docker compose up -d
```

## 配置

| 变量                          | 默认值                 | 说明                                            |
| ----------------------------- | ---------------------- | ----------------------------------------------- |
| `MIONOTE_PATH`                | 必填；容器内为 `/data` | 笔记、附件与 SQLite 数据库的根目录。            |
| `MIONOTE_SECRET_KEY`          | 必填                   | 用于签发 JWT 的随机密钥，应使用足够长的随机值。 |
| `MIONOTE_HOST`                | `0.0.0.0`              | 服务监听地址。                                  |
| `MIONOTE_PORT`                | `4233`                 | 服务监听端口。                                  |
| `MIONOTE_REGISTRATION_OPEN`   | `true`                 | 是否允许创建新账户。                            |
| `MIONOTE_SESSION_EXPIRY_DAYS` | `30`                   | 登录会话有效期，单位为天。                      |
| `MIONOTE_PATH_PREFIX`         | 空                     | 反向代理子路径，例如 `/notes`。                 |

服务日志默认输出 `INFO` 级别的启动、请求结果和写入事件到标准输出；可通过 `RUST_LOG` 使用 `tracing` 过滤规则调整级别，例如 `RUST_LOG=debug`。日志不会记录认证凭据、令牌、笔记内容或动态路由参数。

## 子路径部署

当应用由反向代理暴露在 `https://example.com/notes` 时，设置：

```shell
MIONOTE_PATH_PREFIX=/notes
```

反向代理需要将 `/notes` 及其全部子路径原样转发给 MioNote，不要在转发时剥离该前缀。

## 开发

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

如需前端热更新，在另一个终端启动后端和 Vite 开发服务器：

```shell
MIONOTE_PATH=./data \
MIONOTE_SECRET_KEY=development-secret \
MIONOTE_PORT=4234 \
cargo run

MIONOTE_DEV_API_URL=http://127.0.0.1:4234 npm run dev
```

前端源码位于 `client/src/`，Rust 服务位于 `src/`。后端会直接提供 `client/dist` 中已构建的前端文件；Vite 开发服务器默认监听 `http://127.0.0.1:5173/`。

## 测试与 API 文档

```shell
cargo test
```

运行中的服务提供 API 索引和 OpenAPI 文档：

- `http://localhost:4233/docs`
- `http://localhost:4233/openapi.json`

使用了 `MIONOTE_PATH_PREFIX` 时，请在上述地址前加上对应前缀。

## 备份与恢复

备份 `MIONOTE_PATH` 下的整个目录即可同时备份账户数据库、Markdown 笔记和附件。恢复时，在服务停止状态下用备份目录替换数据目录，并保持容器运行用户对该目录具有读写权限。
