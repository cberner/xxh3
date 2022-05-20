[![Crates.io](https://img.shields.io/crates/v/xxh3.svg)](https://crates.io/crates/xxh3)
[![Documentation](https://docs.rs/xxh3/badge.svg)](https://docs.rs/xxh3)
[![License](https://img.shields.io/crates/l/xxh3)](https://crates.io/crates/xxh3)
[![dependency status](https://deps.rs/repo/github/cberner/xxh3/status.svg)](https://deps.rs/repo/github/cberner/xxh3)

# xxh3
Rust implementation of the XXH3 hash function.

## FAQ
1. How is this crate different than `twox-hash` or `xxhash-rust`?
   This crate has fewer hash algorithms (only XXH3 64bit and 128bit), but supports runtime detection of AVX2 & NEON
   instructions and therefore may be faster. I designed it for use in [redb](https://github.com/cberner/redb).

## License

Licensed under either of

* [Apache License, Version 2.0](LICENSE-APACHE)
* [MIT License](LICENSE-MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
