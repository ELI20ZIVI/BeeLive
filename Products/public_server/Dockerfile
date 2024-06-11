FROM rust:latest

WORKDIR /usr/src/beelive_public

COPY . .

RUN cargo install --path . --target-dir /var/cache/beelive_public

CMD ["public_server"]

EXPOSE 8080
