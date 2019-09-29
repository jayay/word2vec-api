# Word2Vec-api
[![Build Status](https://travis-ci.org/jayay/word2vec-api.svg?branch=master)](https://travis-ci.org/jayay/word2vec-api) [![Coverage Status](https://coveralls.io/repos/github/jayay/word2vec-api/badge.svg?branch=master)](https://coveralls.io/github/jayay/word2vec-api?branch=master) [![dependency status](https://deps.rs/repo/github/jayay/word2vec-api/status.svg)](https://deps.rs/repo/github/jayay/word2vec-api) ![GitHub](https://img.shields.io/github/license/jayay/word2vec-api)

In-memory HTTP service for https://github.com/DimaKudosh/word2vec

This repository uses an experimental fork of DimaKudosh/word2vec with SIMD extensions.


## Installation
Please make sure to set the rust toolchain to nightly first! Then run

```
git clone https://github.com/jayay/word2vec-api.git
cd word2vec-api
cargo run --release -- path/to/model.bin
```
