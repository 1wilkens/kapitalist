FROM rustlang/rust:nightly

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

WORKDIR /usr/src/app
COPY wait-for-port.sh .
COPY .env .

EXPOSE 3000
VOLUME ["/usr/local/cargo"]