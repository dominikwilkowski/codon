# Codon

> A sequence of three nucleotides in mRNA that specifies a particular amino acid or termination signal during protein synthesis.


## Running the project

```sh
cargo leptos watch
```


### Required Tools

- `nightly` Rust
- `cargo-generate`
- `sass`

If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` – make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` – add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` – install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` – install `dart-sass` (should be optional in future)


### Compiling for Release

```sh
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site


### Testing Your Project

```sh
cargo leptos end-to-end
```

```sh
cargo leptos end-to-end --release
```


## Licensing

Copyleft (c) 2023
Licensed under the [GNU GPL-3.0-or-later](https://github.com/dominikwilkowski/codon/blob/main/LICENSE).

