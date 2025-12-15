use serde::Deserialize;
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
}

#[derive(Debug, Deserialize, Clone)]
pub struct McpConfig {
    pub transport: String,
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = config::Config::builder()
            // Start with default values
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8765)?
            .set_default("mcp.transport", "stdio")?
            .set_default("mcp.port", 3000)?
            
            // Layer 1: config file (if exists)
            .add_source(config::File::with_name("config/default").required(false))
            .add_source(config::File::with_name(&format!("config/{}", run_mode)).required(false))
            
            // Layer 2: Environment variables (MOUCHAK_SERVER__PORT=9000)
            .add_source(config::Environment::with_prefix("MOUCHAK").separator("__"))
            
            .build()?;

        s.try_deserialize()
    }
}
