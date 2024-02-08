use pyo3::prelude::*;

use rand::random;

/// Returns the sum of two unsigned integers
#[pyfunction]
fn add(left: usize, right: usize) -> PyResult<usize> {
    Ok(left + right)
}

/// Returns a random u8
#[pyfunction]
fn fake_u8() -> PyResult<u8> {
    Ok(random::<u8>())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn fossair(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(fake_u8, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2).unwrap();
        assert_eq!(result, 4);
    }
}
