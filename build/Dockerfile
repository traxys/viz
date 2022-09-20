FROM rust:1.63

RUN apt-get update -y && apt-get install -y pandoc
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
