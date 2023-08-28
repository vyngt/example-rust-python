use pyo3::prelude::*;

pub mod simple_cal;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn rust_python_api(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simple_cal::sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(simple_cal::two_sum, m)?)?;
    Ok(())
}
