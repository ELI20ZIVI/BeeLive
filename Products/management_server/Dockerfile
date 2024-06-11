FROM rust:latest

WORKDIR /usr/src/beelive_management

COPY . .

RUN cargo install --path . --target-dir /var/cache/beelive_management

WORKDIR /usr/share/beelive_management

COPY ./assets .

CMD ["management_server"]

EXPOSE 8080
