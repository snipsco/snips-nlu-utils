use std::borrow::Borrow;
use std::str::FromStr;

use cpython::{Python, PyResult, PyUnicode, PyList, PythonObject};
use snips_nlu_utils::language::Language;
use snips_nlu_utils::token as token_utils;

use binding_utils::{IntoPyDict, PyResultBinding};

pub fn tokenize(py: Python, input: PyUnicode, language: PyUnicode) -> PyResult<PyList> {
    let language = Language::from_str(language.to_string(py)?.borrow()).py_result(py)?;
    let tokens = token_utils::tokenize(input.to_string(py)?.borrow(), language).into_iter()
        .map(|token| Ok(token.into_py_dict(py)?.into_object()))
        .collect::<PyResult<Vec<_>>>()?;
    Ok(PyList::new(py, tokens.as_slice()))
}

pub fn tokenize_light(py: Python, input: PyUnicode, language: PyUnicode) -> PyResult<PyList> {
    let language = Language::from_str(language.to_string(py)?.borrow()).py_result(py)?;
    let tokens: Vec<_> = token_utils::tokenize_light(input.to_string(py)?.borrow(), language).into_iter()
        .map(|token_str| PyUnicode::new(py, &token_str.to_string()).into_object())
        .collect();
    Ok(PyList::new(py, tokens.as_slice()))
}

pub fn compute_all_ngrams(py: Python, tokens: PyList, max_ngram_size: i32) -> PyResult<PyList> {
    let rust_tokens = tokens.iter(py)
        .map(|token| Ok(token.cast_into::<PyUnicode>(py)?.to_string(py)?.to_string()))
        .collect::<PyResult<Vec<_>>>()?;
    let str_tokens = rust_tokens.iter().map(|t| &**t).collect::<Vec<_>>();
    let ngrams = token_utils::compute_all_ngrams(str_tokens.as_slice(), max_ngram_size as usize).into_iter()
        .map(|ngrams| Ok(ngrams.into_py_dict(py)?.into_object()))
        .collect::<PyResult<Vec<_>>>()?;
    Ok(PyList::new(py, ngrams.as_slice()))
}
