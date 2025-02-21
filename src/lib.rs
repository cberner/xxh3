#![deny(clippy::cast_possible_truncation)]
#![deny(clippy::cast_possible_wrap)]
#![deny(clippy::cast_precision_loss)]
#![deny(clippy::cast_sign_loss)]

#[cfg(feature = "python")]
pub use crate::python::xxh3;

#[cfg(feature = "python")]
mod python;
#[cfg(not(feature = "python"))]
mod xxh3;

#[cfg(not(feature = "python"))]
pub use crate::xxh3::{hash64_with_seed, hash128_with_seed};
