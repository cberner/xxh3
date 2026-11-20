use std::borrow::Cow;

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyBytes};

/// Inputs at least this long are hashed with the GIL released, so that other Python threads can
/// run meanwhile. Releasing and reacquiring the GIL costs more than hashing a shorter input.
const RELEASE_GIL_MIN_LEN: usize = 64 * 1024;

/// Accepts only `bytes` and `bytearray`. pyo3's `Cow<[u8]>` extraction also accepts any sequence
/// of ints (e.g. `list` or `memoryview`), copying it element by element.
fn extract_data<'a>(data: &'a Bound<'_, PyAny>) -> PyResult<Cow<'a, [u8]>> {
    if let Ok(bytes) = data.cast::<PyBytes>() {
        Ok(Cow::Borrowed(bytes.as_bytes()))
    } else if let Ok(byte_array) = data.cast::<PyByteArray>() {
        // Copied, because the contents could be modified while the GIL is released
        Ok(Cow::Owned(byte_array.to_vec()))
    } else {
        Err(PyTypeError::new_err(format!(
            "expected bytes or bytearray, got {}",
            data.get_type().name()?
        )))
    }
}

fn hash<T: Send>(py: Python<'_>, data: &[u8], hasher: impl FnOnce(&[u8]) -> T + Send) -> T {
    if data.len() >= RELEASE_GIL_MIN_LEN {
        py.detach(|| hasher(data))
    } else {
        hasher(data)
    }
}

/// Returns the 64-bit XXH3 hash of `data` (`bytes` or `bytearray`), as an int.
#[pyfunction]
#[pyo3(signature = (data, seed = 0))]
fn xxh3_64(py: Python<'_>, data: &Bound<'_, PyAny>, seed: u64) -> PyResult<u64> {
    let data = extract_data(data)?;
    Ok(hash(py, &data, |data| ::xxh3::hash64_with_seed(data, seed)))
}

/// Returns the 128-bit XXH3 hash of `data` (`bytes` or `bytearray`), as an int.
#[pyfunction]
#[pyo3(signature = (data, seed = 0))]
fn xxh3_128(py: Python<'_>, data: &Bound<'_, PyAny>, seed: u64) -> PyResult<u128> {
    let data = extract_data(data)?;
    Ok(hash(py, &data, |data| {
        ::xxh3::hash128_with_seed(data, seed)
    }))
}

/// XXH3 hash function
#[pymodule]
fn xxh3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(xxh3_64, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_128, m)?)?;
    Ok(())
}
