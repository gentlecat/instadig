FROM rust:bookworm AS builder
WORKDIR /usr/src/instadig
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/instadig /usr/local/bin/instadig
CMD ["instadig"]
