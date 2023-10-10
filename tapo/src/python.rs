//! Python utilities.

use pyo3::types::{PyDict, PyList};
use pyo3::{IntoPy, Py, PyResult, Python, ToPyObject};
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

    Ok(dict.into_py(py))
}

fn map_value(py: Python, value: &Value) -> PyResult<impl ToPyObject> {
    let mapped_value = match value {
        Value::Object(_) => serde_object_to_py_dict(py, value)?.to_object(py),
        Value::Array(value) => {
            let array = PyList::empty(py);

            for item in value {
                let mapped_item = map_value(py, item)?;
                array.append(mapped_item)?;
            }

            array.to_object(py)
        }
        Value::String(value) => ToPyObject::to_object(value, py),
        Value::Bool(value) => ToPyObject::to_object(value, py),
        Value::Number(value) => {
            if let Some(ref value) = value.as_i64() {
                ToPyObject::to_object(value, py)
            } else if let Some(ref value) = value.as_u64() {
                ToPyObject::to_object(value, py)
            } else if let Some(ref value) = value.as_f64() {
                ToPyObject::to_object(value, py)
            } else {
                todo!()
            }
        }
        Value::Null => py.None(),
    };

    Ok(mapped_value)
}
