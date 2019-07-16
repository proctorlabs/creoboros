FROM rust:latest

#Install rust deps
RUN rustup update && \
    rustup component add rls rust-analysis rust-src rustfmt && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo install cargo-make

#Install other tools
RUN apt-get update && apt-get -y install git procps lldb-3.9 fish && \
    apt-get autoremove -y && \
    apt-get clean -y && \
    rm -rf /var/lib/apt/lists/*

RUN mkdir -p /home/dev && \
    useradd -d /home/dev -u 1000 dev -s /usr/bin/fish && \
    chown dev:dev /home/dev

ENV SHELL /usr/bin/fish
USER dev
