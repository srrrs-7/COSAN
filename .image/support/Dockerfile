FROM rust:latest AS builder

ADD ./ms /ms
WORKDIR /ms/support/support

RUN cargo b --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /ms/support/support/target/release/api /usr/local/bin/api
EXPOSE 8080

CMD [ "/usr/local/bin/api" ]