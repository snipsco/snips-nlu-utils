#[macro_use]
extern crate cpython;
extern crate snips_nlu_utils;

pub mod token;
pub mod string;
mod binding_utils;

use cpython::{PyList, PyUnicode};
use string::{get_shape, normalize, remove_diacritics};
use token::{tokenize, tokenize_light, compute_all_ngrams};

py_module_initializer!(_snips_nlu_utils_py, init_snips_nlu_utils_py, PyInit__snips_nlu_utils_py, |py, m| {
    m.add(py, "tokenize", py_fn!(py, tokenize(input: PyUnicode, language: PyUnicode)))?;
    m.add(py, "tokenize_light", py_fn!(py, tokenize_light(input: PyUnicode, language: PyUnicode)))?;
    m.add(py, "compute_all_ngrams", py_fn!(py, compute_all_ngrams(tokens: PyList, max_ngram_size: i32)))?;
    m.add(py, "normalize", py_fn!(py, normalize(string: PyUnicode)))?;
    m.add(py, "remove_diacritics", py_fn!(py, remove_diacritics(string: PyUnicode)))?;
    m.add(py, "get_shape", py_fn!(py, get_shape(string: PyUnicode)))?;
    Ok(())
});
