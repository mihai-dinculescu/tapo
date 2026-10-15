//! What a real device of one model sends. A fixture holds the UDP discovery
//! response under `discovery_response` and, under the name of each method,
//! the `result` that the device answers that method with. A camera takes its
//! methods inside a `multipleRequest`, and the fixture stores the `result` of
//! each inner method under its own name. The results of `login` are keyed by
//! `sub_method`: `discover` in full, which a camera, never asked to
//! discover, leaves out; `pake_register` without the fields that are
//! computed for each login; and `pake_share`, whose result is computed in
//! full, so the fixture holds only the `error_code` that the device answers
//! a wrong password with. The identifiers in a fixture are made up; the
//! rest is what a real device sent.
//!
//! A fixture holds nothing that a device does not send. What the simulator
//! needs to know beyond that comes from where the fixture lives and what it
//! is named: a fixture in `fixtures/smartcam/` is of a camera or a camera
//! hub, one in `fixtures/smart/` is not, and the name ends in the protocol
//! that the device takes.
//!
//! A fixture is named `<discovery model>_<hardware version>_<firmware
//! version>_<auth protocol>.json`, as in `P110(UK)_1.0_1.4.8_TPAP.json`: the
//! `device_model` of the discovery response, the hardware and firmware
//! versions of the device info, the latter cut at its first space, and the
//! protocol that the device takes: the `encrypt_type` that the discovery
//! response announces, or `TPAP` for a camera, which announces a `tpap`
//! object instead.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
pub struct Fixture {
    /// Whether the device is a camera or a camera hub: whether the fixture
    /// lives in `fixtures/smartcam/` rather than `fixtures/smart/`.
    #[serde(skip)]
    camera: bool,
    /// The protocol that the device takes, as the name of its fixture says.
    #[serde(skip)]
    auth_protocol: String,
    discovery_response: Value,
    #[serde(flatten)]
    results: BTreeMap<String, Value>,
}

impl Fixture {
    pub fn load(path: &Path) -> Self {
        let json = std::fs::read_to_string(path)
            .unwrap_or_else(|error| panic!("the fixture {} can be read: {error}", path.display()));

        let camera = match path.parent().and_then(Path::file_name) {
            Some(family) if family == "smartcam" => true,
            Some(family) if family == "smart" => false,
            _ => panic!(
                "the fixture {} lives in fixtures/smart/ or fixtures/smartcam/",
                path.display()
            ),
        };

        let auth_protocol = path
            .file_stem()
            .and_then(|name| name.to_str()?.rsplit_once('_'))
            .map(|(_, auth_protocol)| auth_protocol.to_string())
            .unwrap_or_else(|| {
                panic!(
                    "the name of the fixture {} ends in its auth protocol",
                    path.display()
                )
            });

        let fixture: Self = serde_json::from_str(&json).unwrap_or_else(|error| {
            panic!("the fixture {} is valid JSON: {error}", path.display())
        });
        Self {
            camera,
            auth_protocol,
            ..fixture
        }
    }

    /// The same device reached at `ip`, taking requests on `port`.
    pub fn at(mut self, ip: &str, port: u16) -> Self {
        let is_camera = self.is_camera();
        let announced = &mut self.discovery_response["result"];
        announced["ip"] = json!(ip);
        // A camera or a camera hub announces no web server port.
        if !is_camera {
            announced["mgt_encrypt_schm"]["http_port"] = json!(port);
        }
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

    /// The protocol that the device takes, as the name of its fixture says.
    pub fn auth_protocol(&self) -> &str {
        &self.auth_protocol
    }

    /// Whether the device is a camera or a camera hub, as the folder of its
    /// fixture says.
    pub fn is_camera(&self) -> bool {
        self.camera
    }

    /// Whether the device takes TPAP over TLS: `tls` 1 does, `tls` 0 and a
    /// device that announces no `tpap` do not.
    pub fn tpap_tls(&self) -> bool {
        match &self.announced()["tpap"]["tls"] {
            Value::Null => false,
            tls if *tls == 0 => false,
            tls if *tls == 1 => true,
            tls => unimplemented!("the simulator does not serve TPAP with tls {tls}"),
        }
    }

    pub fn discovery_response(&self) -> &Value {
        &self.discovery_response
    }

    /// The `result` of the `discover` login request, which the fixture of a
    /// camera leaves out.
    pub fn discover_result(&self) -> Option<&Value> {
        let result = Some(&self.result("login")["discover"]).filter(|result| result.is_object());
        assert!(
            result.is_some() || self.is_camera(),
            "the fixture of a device that is not a camera has a result for the discover login request"
        );
        result
    }

    /// The `result` of the `pake_register` login request, less the fields
    /// that are computed for each login.
    pub fn pake_register_result(&self) -> &Value {
        let result = &self.result("login")["pake_register"];
        assert!(
            result.is_object(),
            "the fixture has no result for the pake_register login request"
        );
        result
    }

    /// The error code that the device answers `pake_share` with when the
    /// password is wrong.
    pub fn wrong_password_error_code(&self) -> i64 {
        self.result("login")["pake_share"]["error_code"]
            .as_i64()
            .expect("the fixture has the error code of a wrong password")
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
