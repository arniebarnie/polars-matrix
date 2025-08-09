#![allow(dead_code)]

use pyo3::prelude::*;

mod matrix;
mod expressions;

#[pymodule]
fn _lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
