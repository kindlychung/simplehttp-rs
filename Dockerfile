FROM rust:1-slim-bookworm AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/release/simplehttp-rs /root/
RUN chmod a+x /root/simplehttp-rs
CMD ["/root/simplehttp-rs"]
