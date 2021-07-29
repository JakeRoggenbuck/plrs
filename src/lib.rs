use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use tokens::Tokens;

pub mod tokenizer;
pub mod tokens;

#[pyclass]
#[derive(PartialEq, Debug)]
pub struct Token {
    part: String,
    token: Tokens,
}
