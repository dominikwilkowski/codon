FROM rustlang/rust:nightly-bookworm as builder
RUN apt update && apt install -y bash curl npm libc-dev binaryen \
	protobuf-compiler libssl-dev libprotobuf-dev gcc git g++ libc-dev \
	make binaryen perl

RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-generate
RUN cargo install --locked cargo-leptos --version 0.2.19
RUN cargo install stylance-cli

WORKDIR /work
COPY . .

RUN stylance --output-file ./style/bundle.css ./
ARG DATABASE_URL
RUN cargo leptos build --release

FROM debian:bookworm-slim as runtime
WORKDIR /app
RUN apt-get update -y \
	&& apt-get install -y --no-install-recommends openssl ca-certificates \
	&& apt-get autoremove -y \
	&& apt-get clean -y \
	&& rm -rf /var/lib/apt/lists/*

COPY --from=builder /work/target/release/codon /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT=./site
ENV UPLOAD_ROOT=/app/site/upload_media/
EXPOSE 3000

# Run the server
CMD ["/app/codon"]
