use pyo3::prelude::*;
use pyo3::wrap_pymodule;

mod utils;
use utils::*;

/*
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
 */

#[pymodule]
fn libalpha(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    m.add_wrapped(wrap_pymodule!(utils))?;
    Ok(())
}
