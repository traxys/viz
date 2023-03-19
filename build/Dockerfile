FROM rust:1.68

RUN cargo install sccache

ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache

RUN apt-get update -y && apt-get install -y pandoc
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
