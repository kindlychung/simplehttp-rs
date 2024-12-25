FROM rust:1-slim-bookworm AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /root/
COPY --from=builder /usr/src/app/target/release/simplehttp-rs .
CMD ["./simplehttp-rs "]
