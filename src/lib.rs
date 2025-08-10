#![deny(clippy::all, clippy::pedantic, clippy::disallowed_methods)]
#![deny(clippy::cast_possible_truncation)]
#![deny(clippy::cast_possible_wrap)]
#![deny(clippy::cast_precision_loss)]
#![deny(clippy::cast_sign_loss)]
#![deny(clippy::cast_lossless)]
#![deny(clippy::semicolon_if_nothing_returned)]
#![deny(clippy::ptr_as_ptr)]
// We use intrinsics without alignment requirements, so these casts are all just fine
#![allow(clippy::cast_ptr_alignment)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::similar_names)]

mod xxh3;
pub use crate::xxh3::{hash64_with_seed, hash128_with_seed};
