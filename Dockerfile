FROM alpine:3.19 as build

LABEL org.opencontainers.image.source=https://github.com/rust-lang/docker-rust

RUN apk add --no-cache \
        ca-certificates \
        gcc

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.82.0

RUN set -eux; \
    apkArch="$(apk --print-arch)"; \
    case "$apkArch" in \
        x86_64) rustArch='x86_64-unknown-linux-musl'; rustupSha256='1455d1df3825c5f24ba06d9dd1c7052908272a2cae9aa749ea49d67acbe22b47' ;; \
        aarch64) rustArch='aarch64-unknown-linux-musl'; rustupSha256='7087ada906cd27a00c8e0323401a46804a03a742bd07811da6dead016617cc64' ;; \
        *) echo >&2 "unsupported architecture: $apkArch"; exit 1 ;; \
    esac; \
    url="https://static.rust-lang.org/rustup/archive/1.27.1/${rustArch}/rustup-init"; \
    wget "$url"; \
    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host ${rustArch}; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

RUN curl https://sh.rustup.rs/ -sSf | sh
RUN apk add --no-cache \
    musl-dev \
    gcc \
    build-base
RUN rustup toolchain install stable

WORKDIR /home/crawler

COPY . ./
RUN cargo build --package omicron_crawler --bin omicron_crawler_server --release

FROM alpine:3.19 as stage

USER root
RUN apk add --no-cache chromium-chromedriver unzip curl
RUN curl -o chrome-linux64 https://storage.googleapis.com/chrome-for-testing-public/130.0.6723.91/linux64/chrome-linux64.zip
RUN unzip chrome-linux64 -d ./chrome
ENV PATH=/chrome/chrome-linux64:$PATH
RUN chmod 777 /usr/bin/chromedriver 

EXPOSE 9515

COPY --from=build /home/crawler/target/release/omicron_crawler_server ./home/build/
COPY --from=build /home/crawler/.env ./home/build

