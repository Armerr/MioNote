FROM --platform=$BUILDPLATFORM node:24-alpine AS frontend-build

WORKDIR /build

COPY .htmlnanorc \
    package.json \
    package-lock.json \
    postcss.config.js \
    tailwind.config.js \
    vite.config.js \
    ./

RUN npm ci

COPY client ./client
RUN npm run build

FROM rust:1.91-slim-bookworm AS rust-build

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked

FROM debian:bookworm-slim

ENV PUID=1000 \
    PGID=1000 \
    EXEC_TOOL=gosu \
    MIONOTE_HOST=0.0.0.0 \
    MIONOTE_PORT=4233 \
    APP_PATH=/app \
    MIONOTE_PATH=/data

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates gosu \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir -p ${APP_PATH} ${MIONOTE_PATH}

WORKDIR ${APP_PATH}

COPY --from=rust-build /build/target/release/mionote /usr/local/bin/mionote
COPY --from=frontend-build --chmod=755 /build/client/dist ./client/dist

COPY entrypoint.sh /
RUN chmod +x /entrypoint.sh

VOLUME /data
EXPOSE ${MIONOTE_PORT}/tcp
ENTRYPOINT [ "/entrypoint.sh" ]
