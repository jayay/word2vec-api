FROM rustlang/rust:nightly

WORKDIR /usr/src/word2vec-api
COPY . .

RUN cargo install --path . --root /usr/local

CMD ["word2vec-api"]
