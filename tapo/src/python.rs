use pyo3::{types::PyDict, PyResult, Python};
use serde_json::{Map, Value};

pub fn serde_object_to_py_dict<'a>(
    py: Python<'a>,
    object: &Map<String, Value>,
) -> PyResult<&'a PyDict> {
    let dict = PyDict::new(py);

    for (key, value) in object {
        match value {
            Value::Object(value) => {
                dict.set_item(key, serde_object_to_py_dict(py, value)?)?;
            }
            Value::Array(_) => {
                todo!();
            }
            Value::String(value) => {
                dict.set_item(key, value)?;
            }
            Value::Bool(value) => {
                dict.set_item(key, value)?;
            }
            Value::Number(value) => {
                if let Some(value) = value.as_i64() {
                    dict.set_item(key, value)?;
                } else if let Some(value) = value.as_u64() {
                    dict.set_item(key, value)?;
                } else if let Some(value) = value.as_f64() {
                    dict.set_item(key, value)?;
                }
            }
            Value::Null => {}
        }
    }

    Ok(dict)
}
