FROM rust:1.67-slim-bullseye as dev

WORKDIR /app

RUN apt-get update && apt-get -y install git

COPY . .

CMD ["cargo", "run"]


FROM rust:1.67-slim-bullseye as builder

WORKDIR /app

RUN apt-get update && \
    apt-get install -y \
      musl-tools
RUN rustup target add x86_64-unknown-linux-musl
COPY Cargo.toml Cargo.lock ./
COPY src src

RUN cargo build --release --target=x86_64-unknown-linux-musl


FROM scratch as runner

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rusq .

CMD ["./rusq"]
