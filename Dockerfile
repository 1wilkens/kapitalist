FROM rust:1.31

RUN cargo install -f cargo-watch
RUN cargo install -f diesel_cli --no-default-features --features postgres

WORKDIR /usr/src/kapitalist
COPY wait-for-port.sh .

EXPOSE 5454
VOLUME ["/usr/local/cargo"]
