use std::fmt;

use log::debug;
use serde::Serialize;

use crate::encryption::sha_digest_username;

#[derive(Serialize)]
pub struct LoginDeviceParams {
    username: String,
    password: String,
}

impl LoginDeviceParams {
    pub fn new(username: &str, password: &str) -> anyhow::Result<Self> {
        let username_digest = sha_digest_username(username)?;
        debug!("Username digest: {username_digest}");

        Ok(Self {
            username: base64::encode(username_digest),
            password: base64::encode(password),
        })
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
