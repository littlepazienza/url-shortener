FROM rust as blog-server
USER root
COPY ./ ./
RUN rustup toolchain install nightly-2022-08-11
RUN rustup run nightly-2022-08-11 cargo -V
RUN rustup run nightly-2022-08-11 cargo test
RUN rustup run nightly-2022-08-11 cargo build