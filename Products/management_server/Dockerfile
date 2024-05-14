FROM rust:latest

WORKDIR /usr/local/src/management_server

COPY . .

RUN cargo install --path .

CMD ["management_server"]

EXPOSE 8080
