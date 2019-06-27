use std::collections::HashSet;
use pyo3::{
    gc::{PyGCProtocol, PyVisit},
    mapping::PyMappingProtocol,
    prelude::*,
    PyTraverseError,
};

use like_aho_corasick_but_different::SimpleFinder;

#[pyclass]
struct Searcher {
    inner: Box<SimpleFinder<PyObject>>,
}

#[pymethods]
impl Searcher {
    #[new]
    fn new(obj: &PyRawObject, patterns: Vec<(&str, PyObject)>) {
        obj.init({
            Searcher {
                inner: Box::new(SimpleFinder::new(patterns)),
            }
        })
    }

    fn __sizeof__(&self) -> PyResult<usize> {
        use std::mem::size_of;

        Ok(self.inner.heap_bytes() + size_of::<PyObject>() * self.inner.data().len())
    }

    fn search(&self, haystack: &str) -> PyResult<Vec<PyObject>> {
        use pyo3::AsPyPointer;

        let results: Vec<_> = self.inner.find_all(haystack)
                                        .map(|(_, v)| v)
                                        .collect();
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
        Ok(self.inner.pattern_count())
    }
}

#[pyproto]
impl PyGCProtocol for Searcher {
    fn __traverse__(&self, visit: PyVisit) -> Result<(), PyTraverseError> {
        for obj in self.inner.data().values() {
            visit.call(obj)?;
        }
        Ok(())
    }

    fn __clear__(&mut self) {
        // ...
        // I think this can just be a noop
        // maybe if you do: l = []; s = Searcher([("a", l)]); l.append(s);
        // it will create a cycle
        // I say: don't do that...
    }
}

#[pymodule]
fn lacbd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Searcher>()?;

    Ok(())
}
