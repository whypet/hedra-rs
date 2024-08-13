# hedra

A fast SIMD-optimized Rust 3D software renderer on the CPU.

## Building

If you wish to enable SIMD with the `simd` feature, you'll need to install the Rust nightly toolchain as [`portable_simd`](https://github.com/rust-lang/rust/issues/86656) is used:

`rustup toolchain install nightly`

Then run `cargo build` in the root directory.

> [!NOTE]
> Please note that some parts of hedra that make use of SIMD won't have a scalar equivalent as vectorized code is a bigger priority.
>
> Additionally, a Rust nightly compiler might be needed in the future regardless of whether the SIMD feature is enabled, as hedra tries to make use of compile-time tricks as much as possible for performance, some of which may be unstable Rust RFC features.
