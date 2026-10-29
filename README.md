 # Numerical Integration in Rust

## Getting started

You will need [Rust](https://www.rust-lang.org/tools/install) and Cargo installed.

```sh
git clone <repository-url>
cd numintegration
cargo run --bin lrs
```

The available binaries are `lrs` (left Riemann sum), `rrs` (right Riemann
sum), `m` (midpoint Riemann sum), `s` (Simpson's rule), and `t` (trapezoidal
rule). Select the method with `cargo run --bin <name>`.

Run the test suite with:

```sh
cargo test
```
