use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use toy_json;

#[pyfunction]
pub fn format(s: &str, indent: usize) -> PyResult<String> {
    let opt = toy_json::parse(s);
    match opt {
        Some(Ok(value)) => {
            let dump_options = toy_json::ast::DumpOptions {
                color: false,
                pretty_print: if indent == 0 { false } else { true },
                indent,
            };
            Ok(value.dump(&dump_options))
        }
        Some(Err(e)) => Err(PyValueError::new_err(format!("{}", e))),
        None => Err(PyValueError::new_err("NoTokenFound")),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn toy_json_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(format, m)?)?;
    Ok(())
}
