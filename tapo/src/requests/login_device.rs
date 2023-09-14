use std::fmt;

use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct LoginDeviceParams {
    username: String,
    password: String,
}

impl LoginDeviceParams {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

impl fmt::Debug for LoginDeviceParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"LoginDeviceParams {{ username: "{}", password: "OBSCURED" }}"#,
            self.username,
        )
    }
}
