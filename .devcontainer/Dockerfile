FROM mcr.microsoft.com/devcontainers/rust:latest

RUN apt-get update
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN rustup toolchain install nightly
RUN rustup target add wasm32-unknown-unknown nightly
RUN rustup default nightly
RUN cargo install trunk wasm-bindgen-cli