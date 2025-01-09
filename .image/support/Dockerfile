FROM rust:latest AS builder

RUN apt-get update && \
    apt-get install -y --no-install-recommends mold && \
    rm -rf /var/lib/apt/lists/*

ADD ./ms /ms
WORKDIR /ms/support/support

ENV RUSTFLAGS="-C link-arg=-fuse-ld=mold"

RUN cargo install cargo-chef && cargo chef prepare --recipe-path recipe.toml
RUN cargo build --release --bin api

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /ms/support/support/target/release/api /usr/local/bin/api
EXPOSE 8080

CMD [ "/usr/local/bin/api" ]