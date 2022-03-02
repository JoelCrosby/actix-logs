# build stage
FROM rust:slim as build

# install libpq, libsqlite and create new empty binary project
RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq-dev; \
    rm -rf /var/lib/apt/lists/*; \
    USER=root cargo new --bin app
WORKDIR /app

# copy manifests
COPY ./Cargo.toml ./Cargo.toml

# build this project to cache dependencies
RUN cargo build; \
    rm src/*.rs

# copy project source and necessary files
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .

# add .env and secret.key for Docker env
RUN touch .env;

# rebuild app with project source
RUN rm ./target/debug/deps/actix-logs*; \
    cargo build

# deploy stage
FROM debian:buster-slim

# create app directory
RUN mkdir app
WORKDIR /app

# install libpq
RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq5; \
    rm -rf /var/lib/apt/lists/*

# copy binary and configuration files
COPY --from=build /app/target/debug/actix-logs .
COPY --from=build /app/.env .
COPY --from=build /app/diesel.toml .
COPY ./wait-for-it.sh .

# expose port
EXPOSE 8000

# run the binary
CMD ["./wait-for-it.sh", "db:5432", "--", "/app/actix-logs"]
