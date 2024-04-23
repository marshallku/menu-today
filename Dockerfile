FROM rust:1.73-alpine AS base

WORKDIR /usr/src/menu-today

RUN set -eux; \
    apk add --no-cache musl-dev pkgconfig libressl-dev; \
    rm -rf $CARGO_HOME/registry

COPY Cargo.* .

RUN mkdir src && \
    echo 'fn main() {println!("Hello, world!");}' > src/main.rs && \
    cargo build --release && \
    rm target/release/menu-today* && \
    rm target/release/deps/menu_today* && \
    rm -rf src

FROM base AS builder

COPY src src
RUN cargo build --release

FROM alpine:3.14

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/menu-today/target/release/menu-today .

EXPOSE 41890

CMD ["./menu-today"]