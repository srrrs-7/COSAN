FROM rust:slim-bookworm AS builder
RUN apt-get update && \
    apt-get install -y --no-install-recommends mold && \
    rm -rf /var/lib/apt/lists/*

ADD ./ms /ms
WORKDIR /ms/support/support

ENV RUSTFLAGS="-C link-arg=-fuse-ld=mold"
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --bin api

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /ms/support/support/target/release/api /usr/local/bin/api
EXPOSE 8080

CMD [ "/usr/local/bin/api" ]