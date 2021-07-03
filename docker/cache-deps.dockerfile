FROM ubuntu:20.04 AS tdlib-builder

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y make git zlib1g-dev libssl-dev gperf php-cli cmake g++
RUN git clone https://github.com/tdlib/td.git && \
    cd td && \
    rm -rf build && \
    mkdir build && \
    cd build && \
    cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX:PATH=../tdlib -DTD_ENABLE_LTO=ON .. && \
    cmake --build . --target install

FROM rust:1.53-slim as planner
WORKDIR app
# We only pay the installation cost once,
# it will be cached from the second build onwards
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust:1.53-slim as cacher
WORKDIR app
ENV OPENSSL_LIB_DIR="/usr/lib/x86_64-linux-gnu"
ENV OPENSSL_INCLUDE_DIR="/usr/include/openssl"
RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.53-slim as builder
WORKDIR app
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=tdlib-builder td/tdlib /usr/local
RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev
RUN cargo build --release -p interface

FROM rust:1.53-slim as runtime
WORKDIR app
COPY --from=builder /app/target/release/interface /usr/local/bin
ENTRYPOINT ["./usr/local/bin/interface"]
