FROM rust:1.61.0-alpine3.14 as builder

WORKDIR /usr/src/word2vec-api
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src/ src/

RUN apk add --no-cache musl-dev && \
    rustup default nightly
RUN cargo install --path . --root /tmp/release

FROM alpine:latest
WORKDIR /usr
COPY --from=builder /tmp/release/ .
CMD ["./bin/word2vec-api"]
