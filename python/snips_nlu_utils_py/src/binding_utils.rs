use std::ops::Range;

use snips_nlu_utils::token::{Ngrams, Token};
use cpython::{Python, PythonObject, PyResult, PyList, PyDict, PyUnicode, ToPyObject, PyErr};

pub trait PyResultBinding<T> {
    fn py_result(self, py: Python) -> PyResult<T>;
}

impl<T> PyResultBinding<T> for Result<T, String> {
    fn py_result(self, py: Python) -> PyResult<T> {
        self.map_err(|err| PyErr::new::<PyUnicode, _>(py, PyUnicode::new(py, format!("{:?}", err).as_str())))
    }
}

pub trait IntoPyDict {
    fn into_py_dict(self, py: Python) -> PyResult<PyDict>;
}

pub trait IntoPyUnicode {
    fn into_py_unicode(self, py: Python) -> PyResult<PyUnicode>;
}

impl IntoPyUnicode for String {
    fn into_py_unicode(self, py: Python) -> PyResult<PyUnicode> {
        Ok(PyUnicode::new(py, &self))
    }
}

impl IntoPyDict for Range<usize> {
    fn into_py_dict(self, py: Python) -> PyResult<PyDict> {
        let dict = PyDict::new(py);
        dict.set_item(py, "start", self.start)?;
        dict.set_item(py, "end", self.end)?;
        Ok(dict)
    }
}

impl IntoPyDict for Token {
    fn into_py_dict(self, py: Python) -> PyResult<PyDict> {
        let dict = PyDict::new(py);
        dict.set_item(py, "value", self.value.into_py_unicode(py)?)?;
        dict.set_item(py, "range", self.range.into_py_dict(py)?)?;
        dict.set_item(py, "char_range", self.char_range.into_py_dict(py)?)?;
        Ok(dict)
    }
}

impl IntoPyDict for Ngrams {
    fn into_py_dict(self, py: Python) -> PyResult<PyDict> {
        let token_indexes: Vec<_> = self.1.into_iter()
            .map(|index| index.into_py_object(py).into_object())
            .collect();
        let dict = PyDict::new(py);
        dict.set_item(py, "ngram", self.0)?;
        dict.set_item(py, "token_indexes", PyList::new(py, token_indexes.as_slice()))?;
        Ok(dict)
    }
}
