# build container
FROM rustlang/rust:nightly as build

# create a new empty shell project
RUN USER=root cargo new --lib kapitalist
WORKDIR /kapitalist

# copy over manifests
COPY Cargo.toml Cargo.lock ./

# cache dependencies
RUN mkdir src/bin && echo 'fn main() {}' >> src/bin/main.rs \
    && cargo build --release \
    && rm -rf ./src/* \
    && rm ./target/release/deps/*kapitalist-*

# copy source tree
COPY src ./src
COPY migrations ./migrations

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

ENTRYPOINT ["docker-entrypoint.sh"]
CMD ["serve"]
