# build container
FROM rust:1.32 as build

# create a new empty shell project
RUN USER=root cargo new --lib kapitalist
WORKDIR /kapitalist

# copy over manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN mkdir src/bin && echo 'fn main() {}' >> src/bin/main.rs

# cache dependencies
RUN cargo build --release
RUN rm -rf ./src/*
RUN rm ./target/release/deps/*kapitalist*

# copy source tree
COPY ./src ./src
COPY ./migrations ./migrations

# build for release
RUN cargo build --release

# kapitalist container
FROM debian:stretch-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends libpq5 \
    && rm -rf /var/lib/apt/lists/*

# copy the build artifact from the build stage
COPY --from=build /kapitalist/target/release/kapitalist /usr/bin/kapitalist
COPY docker/*.sh /usr/bin/

CMD ["docker-entrypoint.sh"]
