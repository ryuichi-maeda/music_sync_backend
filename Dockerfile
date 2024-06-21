FROM rust:1.77.1

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install cargo-watch && cargo install sqlx-cli

CMD ["cargo", "watch", "-x", "run"]
