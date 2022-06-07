FROM rust:latest

ARG TARGET=x86_64-unknown-linux-gnu

RUN mkdir -p /usr/src/packtool && \
    rustup target add ${TARGET}
WORKDIR /usr/src/packtool
COPY . /usr/src/packtool/
RUN cargo build --release --all-features --target ${TARGET}
