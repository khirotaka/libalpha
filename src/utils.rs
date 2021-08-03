use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/*
#[pyfunction]
fn hello() -> PyResult<()> {
    println!("こんにちは, 世界!");
    Ok(())
}
*/

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn utils(_py: Python, module: &PyModule) -> PyResult<()> {
    //module.add_wrapped(wrap_pyfunction!(hello))?;
    module.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    Ok(())
}