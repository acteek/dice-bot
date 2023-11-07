FROM rust:1.73.0-bookworm as builder
COPY ./ ./
RUN cargo build --release

FROM debian:bookworm-slim as runtime
ENV RUST_BACKTRACE=1
ENV RUST_LOG=info

COPY --from=builder ./target/release/dice-bot /usr/local/bin

RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get install -y ca-certificates && \
    apt-get autoclean && \
    rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["/usr/local/bin/dice-bot"]
