FROM ubuntu:focal as build
MAINTAINER Nicholas Young <hi@secretfader.com>
WORKDIR /usr/src/app
ENV DEBIAN_FRONTEND=noninteractive
ENV PKG_CONFIG_ALLOW_CROSS=1

# Setup musl libc toolchain
ARG RUST_TOOLCHAIN=stable
ARG RUST_TARGET=x86_64-unknown-linux-musl

# Update, upgrade, and install common build dependencies
RUN apt-get update && apt-get -qy upgrade && \
    apt-get -qy install build-essential curl musl-tools libssl-dev pkg-config

# Install the Rust toolchain via rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
  sh -s -- -y --no-modify-path --default-toolchain ${RUST_TOOLCHAIN} && \
  /root/.cargo/bin/rustup target add ${RUST_TARGET}

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY src/ src/
COPY crates/ crates/
RUN /root/.cargo/bin/cargo build --release --target ${RUST_TARGET}

FROM alpine:latest
ARG RUST_TARGET=x86_64-unknown-linux-musl
COPY --from=build /usr/src/app/target/${RUST_TARGET}/release/play/ /usr/bin/play
ENTRYPOINT ["/usr/bin/play"]
