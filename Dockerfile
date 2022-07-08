FROM rust:1.62.0-slim-buster as builder
WORKDIR /usr/src/sfproc
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/sfproc /usr/local/bin/sfproc
ENTRYPOINT ["sfproc"]