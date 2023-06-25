# syntax=docker/dockerfile:1

# Builder image
FROM rust:alpine AS BUILDER
WORKDIR AAAAAAAA
COPY src src
COPY templates templates
COPY Cargo.* .
RUN apk add --no-cache musl-dev
RUN cargo install --target $(uname -m)-unknown-linux-musl --path .

# App image
FROM alpine
COPY --from=BUILDER /usr/local/cargo/bin/AAAAAAAA /usr/local/bin/AAAAAAAA
WORKDIR /AAAAAAAA
COPY config.ini .
CMD ["AAAAAAAA"]
