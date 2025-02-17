FROM rust:latest

WORKDIR /usr/src/instadig
COPY . .

ENV RUST_LOG=INFO

RUN cargo install --path .

CMD ["instadig"]
