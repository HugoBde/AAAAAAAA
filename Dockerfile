# syntax=docker/dockerfile:1

# Builder image
FROM rust:alpine AS BUILDER
RUN apk add --no-cache musl-dev
WORKDIR AAAAAAAA
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
