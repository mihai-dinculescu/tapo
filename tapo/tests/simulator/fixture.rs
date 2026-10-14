//! What a real device of one model sends. A fixture holds the UDP discovery
//! response under `discovery_response` and, under the name of each method,
//! the `result` that the device answers that method with. The results of
//! `login` are keyed by `sub_method`; only `discover` is stored, since the
//! others are computed. The identifiers in a fixture are made up; the rest is
//! what a real device sent.
//!
//! A fixture is named `<discovery model>_<hardware version>_<firmware
//! version>_<auth protocol>.json`, as in `P110(UK)_1.0_1.4.8_TPAP.json`: the
//! `device_model` of the discovery response, the `hw_ver` and the `fw_ver` of
//! the device info, the latter cut at its first space, and the `encrypt_type`
//! that the discovery response announces.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
pub struct Fixture {
    discovery_response: Value,
    #[serde(flatten)]
    results: BTreeMap<String, Value>,
}

impl Fixture {
    pub fn load(path: &Path) -> Self {
        let json = std::fs::read_to_string(path)
            .unwrap_or_else(|error| panic!("the fixture {} can be read: {error}", path.display()));

        serde_json::from_str(&json)
            .unwrap_or_else(|error| panic!("the fixture {} is valid JSON: {error}", path.display()))
    }

    /// The same device reached at `ip`, taking requests on `port`.
    pub fn at(mut self, ip: &str, port: u16) -> Self {
        let announced = &mut self.discovery_response["result"];
        announced["ip"] = json!(ip);
        announced["mgt_encrypt_schm"]["http_port"] = json!(port);
        if announced["tpap"].is_object() {
            announced["tpap"]["port"] = json!(port);
        }
        if let Some(login) = self.results.get_mut("login")
            && login["discover"]["tpap"].is_object()
        {
            login["discover"]["tpap"]["port"] = json!(port);
        }
        if let Some(device_info) = self.results.get_mut("get_device_info") {
            device_info["ip"] = json!(ip);
        }
        self
    }

    /// The protocol that the device announces: `TPAP` or `KLAP`.
    pub fn auth_protocol(&self) -> &str {
        self.announced()["mgt_encrypt_schm"]["encrypt_type"]
            .as_str()
            .expect("the fixture announces an auth protocol")
    }

    pub fn discovery_response(&self) -> &Value {
        &self.discovery_response
    }

    /// The `result` of the `discover` login request.
    pub fn discover_result(&self) -> &Value {
        let result = &self.result("login")["discover"];
        assert!(
            result.is_object(),
            "the fixture has no result for the discover login request"
        );
        result
    }

    /// The `result` that the device answers `method` with.
    pub fn result(&self, method: &str) -> &Value {
        self.results
            .get(method)
            .unwrap_or_else(|| unimplemented!("the fixture has no result for {method}"))
    }

    /// The `result` of the discovery response.
    fn announced(&self) -> &Value {
        &self.discovery_response["result"]
    }
}
