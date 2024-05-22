FROM rust:latest

WORKDIR /usr/src/beelive_management

COPY . .

RUN cargo install --path . --target-dir /var/cache/beelive_management

CMD ["management_server"]

EXPOSE 8080
