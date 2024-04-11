use iriomote_core::IriomoteCore;
use pyo3::{create_exception, prelude::*, PyAny};
use std::sync::Arc;

create_exception!(error, InitializationError, pyo3::exceptions::PyException);
create_exception!(error, ExecutionError, pyo3::exceptions::PyException);

#[pyclass]
pub struct IriomotePy {
    core: Arc<IriomoteCore>,
}

#[pymethods]
impl IriomotePy {
    #[new]
    pub fn new() -> PyResult<Self> {
        match IriomoteCore::new() {
            Ok(core) => Ok(Self {
                core: Arc::new(core),
            }),
            Err(e) => Err(InitializationError::new_err(e.to_string())),
        }
    }

    pub fn execute<'a>(&self, py: Python<'a>, img: Vec<u8>) -> PyResult<&'a PyAny> {
        let core = Arc::clone(&self.core);
        pyo3_asyncio::tokio::future_into_py(py, async move {
            match core.predict(img).await {
                Ok(result) => Ok(result.unwrap()),
                Err(e) => Err(ExecutionError::new_err(e.to_string())),
            }
        })
    }
}
