# Hedra

A fast SIMD-optimized Rust 3D software renderer on the CPU.

## Building

If you wish to enable SIMD, you'll need to install the Rust nightly toolchain as [`portable_simd`](https://github.com/rust-lang/rust/issues/86656) is used:

`rustup toolchain install nightly`

(Please note that some parts of hedra won't have a scalar equivalent.)

Then run `cargo build` in the root directory.
