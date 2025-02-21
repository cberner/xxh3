use pyo3::prelude::*;

#[pymodule]
pub fn xxh3(_m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
