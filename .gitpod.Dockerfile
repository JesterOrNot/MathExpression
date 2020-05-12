FROM gitpod/workspace-full:latest

USER gitpod

RUN ~/.cargo/bin/cargo install cargo-expand \
    && ~/.cargo/bin/rustup toolchain install nightly
