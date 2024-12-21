FROM rust:latest AS builder

ADD ./ms /ms
WORKDIR /ms/support/support

RUN cargo b --release

FROM debian:stable-slim
COPY --from=builder /ms/support/support/target/release/support /usr/local/bin/api
EXPOSE 8080

CMD [ "/usr/local/bin/api" ]