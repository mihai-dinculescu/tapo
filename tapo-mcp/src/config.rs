use serde::Deserialize;

const ENV_PREFIX: &str = "TAPO_MCP";
const DEFAULT_HTTP_ADDR: &str = "127.0.0.1:3000";
const DEFAULT_DISCOVERY_TIMEOUT: u64 = 5;

#[derive(Clone, Deserialize)]
pub struct AppConfig {
    #[serde(default = "AppConfig::default_http_addr")]
    pub http_addr: String,
    pub username: String,
    pub password: String,
    pub discovery_target: String,
    #[serde(default = "AppConfig::default_discovery_timeout")]
    pub discovery_timeout: u64,
    #[serde(default)]
    pub api_key: Option<String>,
}

impl std::fmt::Debug for AppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppConfig")
            .field("http_addr", &self.http_addr)
            .field("username", &"[redacted]")
            .field("password", &"[redacted]")
            .field("discovery_target", &self.discovery_target)
            .field("discovery_timeout", &self.discovery_timeout)
            .field("api_key", &self.api_key.as_ref().map(|_| "[redacted]"))
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

        Ok(config)
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
            "TAPO_MCP_DISCOVERY_TARGET",
            "TAPO_MCP_HTTP_ADDR",
            "TAPO_MCP_DISCOVERY_TIMEOUT",
            "TAPO_MCP_API_KEY",
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
            password: "super_secret".to_string(),
            discovery_target: "192.168.1.255".to_string(),
            discovery_timeout: 5,
            api_key: Some("my-api-key".to_string()),
        };

        let debug = format!("{config:?}");
        assert!(debug.contains("[redacted]"));
        assert!(!debug.contains("user@example.com"));
        assert!(!debug.contains("super_secret"));
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
}
