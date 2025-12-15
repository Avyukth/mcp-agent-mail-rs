use serde::Deserialize;
use config::{Config, File, Environment};
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub mcp: McpConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub auth_hmac: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct McpConfig {
    pub transport: String,
    pub port: u16,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8769,
                auth_hmac: None,
            },
            mcp: McpConfig {
                transport: "stdio".to_string(),
                port: 3000,
            },
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start with defaults
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8769)?
            .set_default("mcp.transport", "stdio")?
            .set_default("mcp.port", 3000)?
            // Merge in config files
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // Environment overrides
            .add_source(Environment::with_prefix("MOUCHAK").separator("__"))
            .build()?;

        s.try_deserialize()
    }
}
