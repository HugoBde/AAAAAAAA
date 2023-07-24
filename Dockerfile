# syntax=docker/dockerfile:1

# Install cargo-chef
FROM rust:alpine AS chef
RUN apk add --no-cache musl-dev
RUN cargo install cargo-chef
WORKDIR /AAAAAAAA

# Prepare the build
FROM chef AS PLANNER
COPY Cargo.* .
COPY src src
RUN cargo chef prepare --recipe-path recipe.json

# Pre build dependencies then build our code
FROM chef AS BUILDER
COPY --from=PLANNER /AAAAAAAA/recipe.json recipe.json
# Pre build
RUN cargo chef cook --release --target $(uname -m)-unknown-linux-musl
COPY src src
COPY templates templates
COPY Cargo.* .
RUN cargo install --target $(uname -m)-unknown-linux-musl --path .

# App image
FROM alpine
WORKDIR /AAAAAAAA
COPY config.ini .
COPY --from=BUILDER /usr/local/cargo/bin/AAAAAAAA /usr/local/bin/AAAAAAAA
CMD ["AAAAAAAA"]
