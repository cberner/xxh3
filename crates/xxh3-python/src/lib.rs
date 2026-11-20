use std::borrow::Cow;

use pyo3::prelude::*;

/// Inputs at least this long are hashed with the GIL released, so that other Python threads can
/// run meanwhile. Releasing and reacquiring the GIL costs more than hashing a shorter input.
const RELEASE_GIL_MIN_LEN: usize = 64 * 1024;

fn hash<T: Send>(py: Python<'_>, data: &[u8], hasher: impl FnOnce(&[u8]) -> T + Send) -> T {
    if data.len() >= RELEASE_GIL_MIN_LEN {
        py.allow_threads(|| hasher(data))
    } else {
        hasher(data)
    }
}

/// Returns the 64-bit XXH3 hash of `data` (`bytes` or `bytearray`), as an int.
#[pyfunction]
#[pyo3(signature = (data, seed = 0))]
fn xxh3_64(py: Python<'_>, data: Cow<'_, [u8]>, seed: u64) -> u64 {
    hash(py, &data, |data| ::xxh3::hash64_with_seed(data, seed))
}

/// Returns the 128-bit XXH3 hash of `data` (`bytes` or `bytearray`), as an int.
#[pyfunction]
#[pyo3(signature = (data, seed = 0))]
fn xxh3_128(py: Python<'_>, data: Cow<'_, [u8]>, seed: u64) -> u128 {
    hash(py, &data, |data| ::xxh3::hash128_with_seed(data, seed))
}

/// XXH3 hash function
#[pymodule]
fn xxh3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(xxh3_64, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_128, m)?)?;
    Ok(())
}
