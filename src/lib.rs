use pyo3::{
    create_exception,
    gc::{PyGCProtocol, PyVisit},
    mapping::PyMappingProtocol,
    prelude::*,
    PyTraverseError,
};
use std::collections::HashSet;

use like_aho_corasick_but_different::SimpleFinder;

#[pyclass]
struct Searcher {
    inner: Option<Box<SimpleFinder<PyObject>>>,
}

create_exception!(lacbd, SearcherDestructed, pyo3::exceptions::Exception);

#[pymethods]
impl Searcher {
    #[new]
    fn new(obj: &PyRawObject, patterns: Vec<(&str, PyObject)>) {
        obj.init({
            Searcher {
                inner: Some(Box::new(SimpleFinder::new(patterns))),
            }
        })
    }

    fn __sizeof__(&self) -> PyResult<usize> {
        use std::mem::size_of;

        let inner = self.inner.as_ref().ok_or_else(|| SearcherDestructed)?;

        Ok(inner.heap_bytes() + size_of::<PyObject>() * inner.data().len())
    }

    fn search(&self, haystack: &str) -> PyResult<Vec<PyObject>> {
        use pyo3::AsPyPointer;

        let inner = self.inner.as_ref().ok_or_else(|| SearcherDestructed)?;

        let results: Vec<_> = inner.find_all(haystack).map(|(_, v)| v).collect();
        let mut seen = HashSet::new();

        let gil = Python::acquire_gil();
        let py = gil.python();

        let results = results
            .into_iter()
            .filter(|o| seen.insert(o.as_ptr()))
            .map(|o| o.clone_ref(py))
            .collect();

        Ok(results)
    }
}

#[pyproto]
impl PyMappingProtocol for Searcher {
    fn __len__(&self) -> PyResult<usize> {
        let inner = self.inner.as_ref().ok_or_else(|| SearcherDestructed)?;

        Ok(inner.pattern_count())
    }
}

#[pyproto]
impl PyGCProtocol for Searcher {
    fn __traverse__(&self, visit: PyVisit) -> Result<(), PyTraverseError> {
        if let Some(inner) = self.inner.as_ref() {
            for obj in inner.data().values() {
                visit.call(obj)?;
            }
        }

        Ok(())
    }

    fn __clear__(&mut self) {
        if let Some(inner) = self.inner.take() {
            let gil = Python::acquire_gil();
            let py = gil.python();

            for obj in inner.data().values() {
                py.release(obj);
            }
        }
    }
}

#[pymodule]
fn lacbd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Searcher>()?;

    Ok(())
}
