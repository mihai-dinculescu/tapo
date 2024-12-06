//! Python utilities.

use pyo3::types::{PyDict, PyDictMethods, PyList, PyListMethods};
use pyo3::{IntoPyObjectExt, Py, PyResult, Python};
use serde_json::Value;

/// Converts a serde object to a Python dictionary.
pub fn serde_object_to_py_dict(py: Python, value: &Value) -> PyResult<Py<PyDict>> {
    let dict = PyDict::new(py);

    if let Some(object) = value.as_object() {
        for (key, value) in object {
            let value_mapped = map_value(py, value)?;
            dict.set_item(key, value_mapped)?;
        }
    }

    Ok(dict.into())
}

fn map_value<'py>(py: Python<'py>, value: &'py Value) -> PyResult<impl IntoPyObjectExt<'py>> {
    let mapped_value = match value {
        Value::Object(_) => serde_object_to_py_dict(py, value)?.into_py_any(py)?,
        Value::Array(value) => {
            let array = PyList::empty(py);

            for item in value {
                let mapped_item = map_value(py, item)?;
                array.append(mapped_item)?;
            }

            array.into_py_any(py)?
        }
        Value::String(value) => IntoPyObjectExt::into_py_any(value, py)?,
        Value::Bool(value) => IntoPyObjectExt::into_py_any(value, py)?,
        Value::Number(value) => {
            if let Some(ref value) = value.as_i64() {
                IntoPyObjectExt::into_py_any(value, py)?
            } else if let Some(ref value) = value.as_u64() {
                IntoPyObjectExt::into_py_any(value, py)?
            } else if let Some(ref value) = value.as_f64() {
                IntoPyObjectExt::into_py_any(value, py)?
            } else {
                todo!()
            }
        }
        Value::Null => py.None(),
    };

    Ok(mapped_value)
}
