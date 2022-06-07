FROM rust:latest

RUN mkdir -p /usr/src/packtool
WORKDIR /usr/src/packtool
COPY . /usr/src/packtool/
RUN cargo build --release --all-features
