use core::hash::Hash;
use pyo3::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
#[pyclass]
struct PyHashable(PyObject);

impl Hash for PyHashable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let value = &self.0;
        Python::with_gil(|py| {
            let val = value.call_method0(py, "__hash__").unwrap();
            let hash: i64 = val.extract(py).unwrap();
            hash.hash(state);
        });
    }
}

impl PartialEq for PyHashable {
    fn eq(&self, other: &Self) -> bool {
        self.0.is(&other.0)
    }
}

impl Eq for PyHashable {}

#[pyclass]
struct Iter {
    inner: std::vec::IntoIter<PyHashable>,
}

#[pymethods]
impl Iter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyObject> {
        slf.inner.next().map(|x| x.0)
    }
}

#[pyclass]
struct AODict(HashMap<PyHashable, PyHashable>);

#[pymethods]
impl AODict {
    #[new]
    fn new() -> Self {
        AODict(HashMap::new())
    }

    fn __setitem__(&mut self, key: PyObject, value: PyObject) {
        let k = PyHashable(key);
        let v = PyHashable(value);
        if !self.0.contains_key(&k) {
            self._setitem(k, v);
        }
    }

    fn _setitem(&mut self, key: PyHashable, value: PyHashable) {
        self.0.insert(key, value);
    }

    fn __getitem__(&self, key: PyObject) -> PyResult<Option<PyObject>> {
        let k = PyHashable(key);
        Ok(self._getitem(&k).map(|x| x.0))
    }

    fn _getitem(&self, key: &PyHashable) -> Option<PyHashable> {
        self.0.get(key).cloned()
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<Iter>> {
        let iter = Iter {
            inner: slf
                .0
                .keys()
                .cloned()
                .collect::<Vec<PyHashable>>()
                .into_iter(),
        };

        Py::new(slf.py(), iter)
    }
}

#[pymodule]
fn aodict(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AODict>()?;
    Ok(())
}
