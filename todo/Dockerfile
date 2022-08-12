FROM rust:1.62 AS builder

# 空のソースでビルドしておき依存クレートのダウンロードなどのレイヤーキャッシュが効きやすいにしておく
WORKDIR /todo
COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release

# 本番のアプリソースで再度ビルドする
COPY ./src ./src
COPY ./templates ./templates
RUN rm -f target/release/deps/todo*
RUN cargo build --release

FROM debian:10.4

COPY --from=builder /todo/target/release/todo /usr/local/bin/todo
CMD ["todo"]
