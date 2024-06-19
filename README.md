# Codon

> A sequence of three nucleotides in mRNA that specifies a particular amino acid
> or termination signal during protein synthesis.

## Running

TODO


## Deployment

TODO


## Development

```sh
cargo leptos watch
```

Run the formatter before committing code:

```sh
leptosfmt */**/*.rs && cargo fmt -- -l && cargo clippy
```


### Setup

Required:
- `nightly` Rust
- `cargo-generate`
- `leptosfmt`

You need to install:
1. `rustup toolchain install nightly --allow-downgrade` – make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` – add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` – install `cargo-generate` binary
4. `cargo install leptosfmt` – install the formatter for the `view!` macros

For the end2end tests:
- `cd end2end && npm i` – To install the dev dependencies
- `npx playwright install --with-deps` – To install the browsers playwright needs
- 


### Compiling for Release

```sh
cargo leptos build --release
```

Will generate your server binary in `target/server/release` and your site
package in `target/site`


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

