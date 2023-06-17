use std::fmt;

use base64::{engine::general_purpose, Engine as _};
use log::debug;
use serde::Serialize;

use crate::encryption::sha_digest_username;

#[derive(Serialize)]
pub(crate) struct LoginDeviceParams {
    username: String,
    password: String,
}

impl LoginDeviceParams {
    pub fn new(username: &str, password: &str) -> Self {
        let username_digest = sha_digest_username(username);
        debug!("Username digest: {username_digest}");

        Self {
            username: general_purpose::STANDARD.encode(username_digest),
            password: general_purpose::STANDARD.encode(password),
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
