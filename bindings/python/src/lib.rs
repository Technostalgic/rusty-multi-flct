use pyo3::prelude::*;

#[pymodule]
fn rustypy_multi_flct(_m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}

/// UNIT TESTS -----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exists() {}
}
