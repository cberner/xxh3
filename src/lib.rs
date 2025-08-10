#![deny(clippy::cast_possible_truncation)]
#![deny(clippy::cast_possible_wrap)]
#![deny(clippy::cast_precision_loss)]
#![deny(clippy::cast_sign_loss)]
#![deny(clippy::cast_lossless)]
#![deny(clippy::semicolon_if_nothing_returned)]

mod xxh3;
pub use crate::xxh3::{hash64_with_seed, hash128_with_seed};
