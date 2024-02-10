FROM rust:latest

WORKDIR /usr/src/instadig
COPY . .

RUN cargo install --path .

CMD ["instadig"]
