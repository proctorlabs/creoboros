FROM rust:latest
ENV SHELL /bin/bash

RUN rustup update && \
    rustup component add rls rust-analysis rust-src rustfmt && \
    apt-get update && apt-get -y install git procps lldb-3.9 && \
    apt-get autoremove -y && \
    apt-get clean -y && \
    rm -rf /var/lib/apt/lists/* && \
    cargo install cargo-make
