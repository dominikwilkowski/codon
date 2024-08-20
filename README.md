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
4. `cargo install --locked cargo-leptos` – install the leptos build tool
5. `cargo install leptosfmt` – install the formatter for the `view!` macros

For the end2end tests:
- `cd end2end && npm i` – To install the dev dependencies
- `npx playwright install --with-deps` – To install the browsers playwright needs

For upgrading to latest leptos:
- `cargo install cargo-generate`, `cargo install cargo-leptos` and `cargo install leptosfmt` to upgrade to latest global installs


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

If a test fails it might still have the leptos server running in the background.
This will cause the next run to also fail because the IP address and port is already being used:
```
called `Result::unwrap()` on an `Err` value: Os { code: 48, kind: AddrInUse, message: "Address already in use" }
```

To avoid this make sure you detect what process is still running and kill it:

```sh
λ ps -e|grep codon
52333 ttys032    0:00.09 target/debug/codon # our process with PID we need to kill
52379 ttys042    0:00.00 grep codon         # this grep search
λ kill 52333
```


## Licensing

Copyleft (c) 2023
Licensed under the [GNU GPL-3.0-or-later](https://github.com/dominikwilkowski/codon/blob/main/LICENSE).
