FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    curl \
    git \
    build-essential \
    clang \
    cmake \
    libssl-dev \
    pkg-config \
    python3 \
    python3-pip \
    hmmer \
    && apt-get clean

# python3 を python にリンク
RUN ln -s /usr/bin/python3 /usr/bin/python

# Rustインストール
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# numpy インストール
RUN pip3 install numpy bio
RUN pip3 install git+https://github.com/oxpig/ANARCI.git

# Rust CLIツールビルド
WORKDIR /app
COPY ./rust /app
RUN cargo build --release

RUN cp target/release/extracdr /usr/local/bin/

WORKDIR /data

ENTRYPOINT ["/bin/bash"]
