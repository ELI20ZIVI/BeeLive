FROM rust:latest

WORKDIR /usr/local/src/public_server

COPY . .

RUN cargo install --path .

CMD ["public_server"]

EXPOSE 8080
