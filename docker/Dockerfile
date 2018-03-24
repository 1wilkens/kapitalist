FROM rustlang/rust:nightly

RUN cargo install -f diesel_cli --no-default-features --features postgres
RUN cargo install -f cargo-watch

WORKDIR /usr/src/app
COPY wait-for-port.sh .

EXPOSE 3000
VOLUME ["/usr/local/cargo"]