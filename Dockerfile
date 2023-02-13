FROM rust:latest

LABEL maintainer="Oliver Wissett"
LABEL version="1.0"

# Install core packages
RUN DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install -y \
    git \
    make \
    pkg-config \
    build-essential \
    autoconf \
    libc++-dev \
    libc++abi-dev \
    check \
    libclang-dev \
    clang
