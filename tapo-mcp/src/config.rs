use serde::Deserialize;
use url::Url;

const ENV_PREFIX: &str = "TAPO_MCP";
const DEFAULT_HTTP_ADDR: &str = "127.0.0.1:3000";
const DEFAULT_DISCOVERY_TIMEOUT: u64 = 5;

#[derive(Clone, Deserialize)]
pub struct AppConfig {
    #[serde(default = "AppConfig::default_http_addr")]
    pub http_addr: String,
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub camera_username: Option<String>,
    #[serde(default)]
    pub camera_password: Option<String>,
    pub discovery_target: String,
    #[serde(default = "AppConfig::default_discovery_timeout")]
    pub discovery_timeout: u64,
    #[serde(default)]
    pub api_key: Option<String>,
    /// Hostnames or `host:port` authorities accepted in the inbound `Host`
    /// header. Empty keeps rmcp's loopback-only default, which blocks DNS
    /// rebinding. Populated from `TAPO_MCP_ALLOWED_HOSTS`.
    #[serde(skip)]
    pub allowed_hosts: Vec<String>,
    /// Base URL clients reach the server at, without a trailing `/`. Enables
    /// short-lived snapshot links. Populated from `TAPO_MCP_PUBLIC_URL`.
    #[serde(default)]
    pub public_url: Option<String>,
}

impl std::fmt::Debug for AppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppConfig")
            .field("http_addr", &self.http_addr)
            .field("username", &"[redacted]")
            .field("password", &"[redacted]")
            .field(
                "camera_username",
                &self.camera_username.as_ref().map(|_| "[redacted]"),
            )
            .field(
                "camera_password",
                &self.camera_password.as_ref().map(|_| "[redacted]"),
            )
            .field("discovery_target", &self.discovery_target)
            .field("discovery_timeout", &self.discovery_timeout)
            .field("api_key", &self.api_key.as_ref().map(|_| "[redacted]"))
            .field("allowed_hosts", &self.allowed_hosts)
            .field("public_url", &self.public_url)
            .finish()
    }
}

impl AppConfig {
    fn default_http_addr() -> String {
        DEFAULT_HTTP_ADDR.to_string()
    }

    fn default_discovery_timeout() -> u64 {
        DEFAULT_DISCOVERY_TIMEOUT
    }

    pub fn from_env() -> Result<Self, config::ConfigError> {
        let required_envs = [
            format!("{ENV_PREFIX}_USERNAME"),
            format!("{ENV_PREFIX}_PASSWORD"),
            format!("{ENV_PREFIX}_DISCOVERY_TARGET"),
        ];

        let missing: Vec<String> = required_envs
            .iter()
            .filter(|name| {
                std::env::var(name)
                    .map(|v| v.trim().is_empty())
                    .unwrap_or(true)
            })
            .cloned()
            .collect();

        if !missing.is_empty() {
            return Err(config::ConfigError::Message(format!(
                "Missing or empty required environment variable(s): {}",
                missing.join(", ")
            )));
        }

        let mut config: Self = config::Config::builder()
            .add_source(config::Environment::default().prefix(ENV_PREFIX))
            .build()?
            .try_deserialize()?;

        config.api_key = config
            .api_key
            .map(|k| k.trim().to_string())
            .filter(|k| !k.is_empty());

        config.camera_username = config
            .camera_username
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty());

        config.camera_password = config
            .camera_password
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty());

        config.allowed_hosts = Self::parse_list_env(&format!("{ENV_PREFIX}_ALLOWED_HOSTS"));

        config.public_url = config
            .public_url
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
            .map(|v| Self::normalize_public_url(&v))
            .transpose()?;

        config.validate_binding_security()?;

        Ok(config)
    }

    /// Validates `TAPO_MCP_PUBLIC_URL` and returns it normalized, without a trailing slash.
    fn normalize_public_url(raw: &str) -> Result<String, config::ConfigError> {
        let invalid = |reason: &str| {
            config::ConfigError::Message(format!(
                "{ENV_PREFIX}_PUBLIC_URL must be an absolute http(s) URL, got '{raw}' ({reason})"
            ))
        };

        let url = Url::parse(raw).map_err(|e| invalid(&e.to_string()))?;
        if !matches!(url.scheme(), "http" | "https") {
            return Err(invalid("scheme must be http or https"));
        }
        if url.host_str().is_none_or(str::is_empty) {
            return Err(invalid("missing host"));
        }
        if url.query().is_some() || url.fragment().is_some() {
            return Err(invalid("query strings and fragments are not supported"));
        }
        if !url.username().is_empty() || url.password().is_some() {
            return Err(config::ConfigError::Message(format!(
                "{ENV_PREFIX}_PUBLIC_URL must not contain a username or password"
            )));
        }

        Ok(url.as_str().trim_end_matches('/').to_string())
    }

    /// Reads a comma-separated env var into a list of trimmed, non-empty entries.
    fn parse_list_env(name: &str) -> Vec<String> {
        std::env::var(name)
            .unwrap_or_default()
            .split(',')
            .map(|entry| entry.trim().to_string())
            .filter(|entry| !entry.is_empty())
            .collect()
    }

    /// True when `http_addr` binds to a loopback interface (or `localhost`).
    fn binds_to_loopback(&self) -> bool {
        if let Ok(socket) = self.http_addr.parse::<std::net::SocketAddr>() {
            return socket.ip().is_loopback();
        }
        // Fall back to parsing `host:port` forms that aren't literal socket
        // addresses (e.g. `localhost:3000`). Unknown hosts are treated as
        // non-loopback so the guard fails safe.
        let host = self
            .http_addr
            .rsplit_once(':')
            .map(|(host, _)| host)
            .unwrap_or(&self.http_addr)
            .trim_start_matches('[')
            .trim_end_matches(']');

        host.eq_ignore_ascii_case("localhost")
            || host
                .parse::<std::net::IpAddr>()
                .map(|ip| ip.is_loopback())
                .unwrap_or(false)
    }

    /// Refuses a network-exposed bind that has no access control, so the
    /// documented deployment cannot silently run unauthenticated.
    fn validate_binding_security(&self) -> Result<(), config::ConfigError> {
        if self.binds_to_loopback() || self.api_key.is_some() {
            return Ok(());
        }

        Err(config::ConfigError::Message(format!(
            "Refusing to start: {ENV_PREFIX}_HTTP_ADDR binds to a non-loopback address ({}) \
             with no {ENV_PREFIX}_API_KEY set, exposing unauthenticated smart-home control to \
             the network. Set {ENV_PREFIX}_API_KEY to require bearer authentication, or bind to \
             a loopback address.",
            self.http_addr
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Env var tests must run serially since they mutate process-wide state.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    /// # Safety
    ///
    /// Callers must hold `ENV_LOCK` to ensure no concurrent env mutation.
    unsafe fn clear_tapo_env() {
        for key in [
            "TAPO_MCP_USERNAME",
            "TAPO_MCP_PASSWORD",
            "TAPO_MCP_CAMERA_USERNAME",
            "TAPO_MCP_CAMERA_PASSWORD",
            "TAPO_MCP_DISCOVERY_TARGET",
            "TAPO_MCP_HTTP_ADDR",
            "TAPO_MCP_DISCOVERY_TIMEOUT",
            "TAPO_MCP_API_KEY",
            "TAPO_MCP_ALLOWED_HOSTS",
            "TAPO_MCP_PUBLIC_URL",
        ] {
            unsafe { std::env::remove_var(key) };
        }
    }

    /// # Safety
    ///
    /// Callers must hold `ENV_LOCK` to ensure no concurrent env mutation.
    unsafe fn set_required_env() {
        unsafe {
            std::env::set_var("TAPO_MCP_USERNAME", "user@example.com");
            std::env::set_var("TAPO_MCP_PASSWORD", "secret");
            std::env::set_var("TAPO_MCP_DISCOVERY_TARGET", "192.168.1.255");
        }
    }

    #[test]
    fn from_env_missing_all_required_vars() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe { clear_tapo_env() };

        let err = AppConfig::from_env().unwrap_err().to_string();
        assert!(
            err.contains("TAPO_MCP_USERNAME"),
            "error should mention USERNAME: {err}"
        );
        assert!(
            err.contains("TAPO_MCP_PASSWORD"),
            "error should mention PASSWORD: {err}"
        );
        assert!(
            err.contains("TAPO_MCP_DISCOVERY_TARGET"),
            "error should mention DISCOVERY_TARGET: {err}"
        );
    }

    #[test]
    fn from_env_missing_one_required_var() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe {
            clear_tapo_env();
            std::env::set_var("TAPO_MCP_USERNAME", "user@example.com");
            std::env::set_var("TAPO_MCP_PASSWORD", "secret");
        }
        // DISCOVERY_TARGET intentionally omitted

        let err = AppConfig::from_env().unwrap_err().to_string();
        assert!(
            err.contains("TAPO_MCP_DISCOVERY_TARGET"),
            "error should mention the missing var: {err}"
        );
        assert!(
            !err.contains("TAPO_MCP_USERNAME"),
            "should not mention present vars: {err}"
        );
    }

    #[test]
    fn from_env_empty_required_var_treated_as_missing() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe {
            clear_tapo_env();
            set_required_env();
            std::env::set_var("TAPO_MCP_USERNAME", "  ");
        }

        let err = AppConfig::from_env().unwrap_err().to_string();
        assert!(
            err.contains("TAPO_MCP_USERNAME"),
            "blank value should be treated as missing: {err}"
        );
    }

    #[test]
    fn debug_redacts_credentials() {
        let config = AppConfig {
            http_addr: "127.0.0.1:3000".to_string(),
            username: "user@example.com".to_string(),
            password: "super-secret".to_string(),
            camera_username: Some("alice@cam".to_string()),
            camera_password: Some("cam-very-secret".to_string()),
            discovery_target: "192.168.1.255".to_string(),
            discovery_timeout: 5,
            api_key: Some("my-api-key".to_string()),
            allowed_hosts: vec![],
            public_url: None,
        };

        let debug = format!("{config:?}");
        assert!(debug.contains("[redacted]"));
        assert!(!debug.contains("user@example.com"));
        assert!(!debug.contains("super-secret"));
        assert!(!debug.contains("alice@cam"));
        assert!(!debug.contains("cam-very-secret"));
        assert!(!debug.contains("my-api-key"));
    }

    #[test]
    fn api_key_empty_normalizes_to_none() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe {
            clear_tapo_env();
            set_required_env();
            std::env::set_var("TAPO_MCP_API_KEY", "   ");
        }

        let config = AppConfig::from_env().unwrap();
        assert!(config.api_key.is_none());
    }

    #[test]
    fn api_key_trims_whitespace() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe {
            clear_tapo_env();
            set_required_env();
            std::env::set_var("TAPO_MCP_API_KEY", "  my-key  ");
        }

        let config = AppConfig::from_env().unwrap();
        assert_eq!(config.api_key.as_deref(), Some("my-key"));
    }

    #[test]
    fn camera_credentials_empty_normalize_to_none() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe {
            clear_tapo_env();
            set_required_env();
            std::env::set_var("TAPO_MCP_CAMERA_USERNAME", "  ");
            std::env::set_var("TAPO_MCP_CAMERA_PASSWORD", "");
        }

        let config = AppConfig::from_env().unwrap();
        assert!(config.camera_username.is_none());
        assert!(config.camera_password.is_none());
    }

    #[test]
    fn camera_credentials_trim_whitespace() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe {
            clear_tapo_env();
            set_required_env();
            std::env::set_var("TAPO_MCP_CAMERA_USERNAME", "  cam_user  ");
            std::env::set_var("TAPO_MCP_CAMERA_PASSWORD", "  cam_pass  ");
        }

        let config = AppConfig::from_env().unwrap();
        assert_eq!(config.camera_username.as_deref(), Some("cam_user"));
        assert_eq!(config.camera_password.as_deref(), Some("cam_pass"));
    }

    #[test]
    fn allowed_hosts_parse_as_list() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe {
            clear_tapo_env();
            set_required_env();
            std::env::set_var(
                "TAPO_MCP_ALLOWED_HOSTS",
                " 192.168.1.50:3000 , tapo.local , ",
            );
            std::env::set_var("TAPO_MCP_API_KEY", "a-key");
        }

        let config = AppConfig::from_env().unwrap();
        assert_eq!(
            config.allowed_hosts,
            vec!["192.168.1.50:3000".to_string(), "tapo.local".to_string()]
        );
    }

    #[test]
    fn allowed_hosts_default_empty() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe {
            clear_tapo_env();
            set_required_env();
        }

        let config = AppConfig::from_env().unwrap();
        assert!(config.allowed_hosts.is_empty());
    }

    #[test]
    fn public_url_strips_trailing_slash() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe {
            clear_tapo_env();
            set_required_env();
            std::env::set_var("TAPO_MCP_PUBLIC_URL", " https://tapo.example.com/ ");
        }

        let config = AppConfig::from_env().unwrap();
        assert_eq!(
            config.public_url.as_deref(),
            Some("https://tapo.example.com")
        );
    }

    #[test]
    fn public_url_empty_normalizes_to_none() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe {
            clear_tapo_env();
            set_required_env();
            std::env::set_var("TAPO_MCP_PUBLIC_URL", "  ");
        }

        let config = AppConfig::from_env().unwrap();
        assert!(config.public_url.is_none());
    }

    #[test]
    fn public_url_without_scheme_is_rejected() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe {
            clear_tapo_env();
            set_required_env();
            std::env::set_var("TAPO_MCP_PUBLIC_URL", "tapo.example.com");
        }

        let err = AppConfig::from_env().unwrap_err().to_string();
        assert!(
            err.contains("TAPO_MCP_PUBLIC_URL must be an absolute http(s) URL"),
            "should reject a URL without a scheme: {err}"
        );
    }

    #[test]
    fn public_url_uppercase_scheme_is_normalized() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe {
            clear_tapo_env();
            set_required_env();
            std::env::set_var("TAPO_MCP_PUBLIC_URL", "HTTPS://Tapo.Example.com/mcp/");
        }

        let config = AppConfig::from_env().unwrap();
        assert_eq!(
            config.public_url.as_deref(),
            Some("https://tapo.example.com/mcp")
        );
    }

    #[test]
    fn public_url_without_host_is_rejected() {
        for value in ["https://", "http:///"] {
            let _lock = ENV_LOCK.lock().unwrap();
            unsafe {
                clear_tapo_env();
                set_required_env();
                std::env::set_var("TAPO_MCP_PUBLIC_URL", value);
            }

            let err = AppConfig::from_env().unwrap_err().to_string();
            assert!(
                err.contains("TAPO_MCP_PUBLIC_URL must be an absolute http(s) URL"),
                "should reject '{value}': {err}"
            );
        }
    }

    #[test]
    fn public_url_with_query_is_rejected() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe {
            clear_tapo_env();
            set_required_env();
            std::env::set_var("TAPO_MCP_PUBLIC_URL", "https://tapo.example.com/?a=1");
        }

        let err = AppConfig::from_env().unwrap_err().to_string();
        assert!(
            err.contains("query strings and fragments are not supported"),
            "should reject a URL with a query: {err}"
        );
    }

    #[test]
    fn public_url_with_credentials_is_rejected() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe {
            clear_tapo_env();
            set_required_env();
            std::env::set_var(
                "TAPO_MCP_PUBLIC_URL",
                "https://admin:secret@tapo.example.com",
            );
        }

        let err = AppConfig::from_env().unwrap_err().to_string();
        assert!(
            err.contains("must not contain a username or password"),
            "should reject a URL with credentials: {err}"
        );
        assert!(
            !err.contains("secret"),
            "error should not echo the password: {err}"
        );
    }

    #[test]
    fn binds_to_loopback_detects_loopback_forms() {
        let loopback = [
            "127.0.0.1:3000",
            "localhost:3000",
            "[::1]:3000",
            "127.0.0.1:80",
        ];
        for addr in loopback {
            let config = config_with_addr(addr);
            assert!(config.binds_to_loopback(), "{addr} should be loopback");
        }

        let exposed = [
            "0.0.0.0:3000",
            "192.168.1.50:3000",
            "[::]:3000",
            "tapo.local:3000",
        ];
        for addr in exposed {
            let config = config_with_addr(addr);
            assert!(!config.binds_to_loopback(), "{addr} should not be loopback");
        }
    }

    #[test]
    fn non_loopback_bind_without_key_is_rejected() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe {
            clear_tapo_env();
            set_required_env();
            std::env::set_var("TAPO_MCP_HTTP_ADDR", "0.0.0.0:3000");
        }

        let err = AppConfig::from_env().unwrap_err().to_string();
        assert!(
            err.contains("non-loopback"),
            "should refuse the exposed unauthenticated bind: {err}"
        );
    }

    #[test]
    fn non_loopback_bind_with_key_is_allowed() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe {
            clear_tapo_env();
            set_required_env();
            std::env::set_var("TAPO_MCP_HTTP_ADDR", "0.0.0.0:3000");
            std::env::set_var("TAPO_MCP_API_KEY", "a-key");
        }

        assert!(AppConfig::from_env().is_ok());
    }

    fn config_with_addr(addr: &str) -> AppConfig {
        AppConfig {
            http_addr: addr.to_string(),
            username: "user@example.com".to_string(),
            password: "secret".to_string(),
            camera_username: None,
            camera_password: None,
            discovery_target: "192.168.1.255".to_string(),
            discovery_timeout: 5,
            api_key: None,
            allowed_hosts: vec![],
            public_url: None,
        }
    }
}
