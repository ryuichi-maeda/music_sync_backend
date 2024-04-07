FROM rust:1.77.1

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install cargo-watch

CMD ["cargo", "watch", "-x", "run"]
