FROM rust:latest AS build
WORKDIR /usr/src
COPY . .
RUN cargo install --path .

FROM arm64v8/debian:buster-slim
RUN apt-get update
COPY --from=build /usr/local/cargo/bin/serv /usr/local/bin/serv
CMD ["serv"]