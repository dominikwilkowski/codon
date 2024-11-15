FROM rustlang/rust:nightly as builder

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

FROM rustlang/rust:nightly as runner

WORKDIR /app

COPY --from=builder /work/target/release/codon /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

EXPOSE $PORT
ENV LEPTOS_SITE_ROOT=./site

CMD ["/app/codon"]
