FROM rust:1.73-alpine AS chef

WORKDIR /usr/src/menu-today

RUN set -eux; \
    apk add --no-cache musl-dev pkgconfig libressl-dev; \
    cargo install cargo-chef; \
    rm -rf $CARGO_HOME/registry

FROM chef as planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /usr/src/menu-today/recipe.json .
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM alpine:3.14

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/menu-today/target/release/menu-today .

EXPOSE 41880

CMD ["./menu-today"]