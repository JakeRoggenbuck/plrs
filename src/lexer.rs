use super::{pyfunction, pymodule, wrap_pyfunction, PyModule, PyResult, Python};

#[pyfunction]
pub fn lexer() -> bool {
    return true;
}

#[pymodule]
fn plrs_lexer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(lexer, m)?)?;

    Ok(())
}
