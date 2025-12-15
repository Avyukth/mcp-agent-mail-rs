use clap::{Args, Parser, Subcommand};
use lib_common::{
    config::{AppConfig, McpConfig, ServerConfig},
    tracing::setup_tracing,
};
use tracing::info;

#[derive(Parser)]
#[command(name = "mcp-agent-mail")]
#[command(about = "Unified Server/CLI for Agent Mail")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Log format: plain or json
    #[arg(long, default_value = "plain", global = true)]
    log_format: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a server (HTTP or MCP)
    Serve(ServeArgs),

    /// Check server health
    Health {
        #[arg(short, long, default_value = "http://localhost:8765")]
        url: String,
    },

    /// Show version info
    Version,
}

#[derive(Args)]
struct ServeArgs {
    #[command(subcommand)]
    command: ServeCommands,
}

#[derive(Subcommand)]
enum ServeCommands {
    /// Start the HTTP API Server
    Http {
        #[arg(short, long)]
        port: Option<u16>,
    },
    /// Start the MCP Server (Stdio or SSE)
    Mcp {
        #[arg(long, default_value = "stdio")]
        transport: String, // stdio, sse
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Parse CLI Args
    let cli = Cli::parse();

    // 2. Setup Tracing
    let json_logs = cli.log_format == "json";
    // For MCP Stdio, we MUST ensure logs go to stderr to avoid corrupting stdout JSON-RPC.
    // lib-common's setup_tracing should handle this or we explicitly set it here.
    // tracing-subscriber fmt layer defaults to stdout. We should probably set it to stderr generally for this CLI.
    // Or at least for stdio mode.
    // Let's modify lib-common tracing or just use library explicitly here.
    // For now, let's assume setup_tracing outputs to stdout which is BAD for stdio.
    // Wait, typical pattern: application logs to stderr, output to stdout.
    
    // Changing output to stderr for safety across the board.
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, fmt, Layer};
    
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug,axum=debug,mcp_agent_mail=debug"));

    let layer = if json_logs {
        fmt::layer().json().with_writer(std::io::stderr).boxed()
    } else {
        fmt::layer().pretty().with_writer(std::io::stderr).boxed()
    };

    tracing_subscriber::registry()
        .with(env_filter)
        .with(layer)
        .try_init()?;

    // 3. Load Config
    // We load config, then override with CLI args if present
    let mut config = AppConfig::load().unwrap_or_else(|e| {
        // Log warning but continue with defaults if config file fails? 
        // AppConfig loads defaults so it might fail only on hard IO errors.
        tracing::warn!("Failed to load config file: {}. Using CLI/Defaults.", e);
        // We can't easily construct a default AppConfig without exposed defaults logic or Default impl.
        // Let's assume AppConfig::load() handles it gracefullly or we crash.
        // Actually AppConfig::load returns Result.
        // Let's panic if we can't load basic config? Or construct a dummy?
        // Let's modify AppConfig to derive Default if possible, but it uses `config` crate.
        // For now, if load fails, we panic.
        panic!("Config load failed: {}", e);
    });

    // 4. Execute Command
    match cli.command {
        Commands::Serve(args) => match args.command {
            ServeCommands::Http { port } => {
                if let Some(p) = port {
                    config.server.port = p;
                }
                info!("Starting HTTP Server...");
                lib_server::run(config.server).await?;
            }
            ServeCommands::Mcp { transport, port } => {
                config.mcp.transport = transport.clone();
                config.mcp.port = port;

                info!("Starting MCP Server ({})", transport);
                match transport.as_str() {
                    "sse" => lib_mcp::run_sse(config.mcp).await?,
                    _ => lib_mcp::run_stdio(config.mcp).await?,
                }
            }
        },
        Commands::Health { url } => {
            info!("Checking health at {}", url);
            let resp = reqwest::get(format!("{}/health", url)).await?;
            if resp.status().is_success() {
                info!("Server is HEALTHY: {}", resp.text().await?);
            } else {
                tracing::error!("Server is UNHEALTHY: Status {}", resp.status());
                std::process::exit(1);
            }
        }
        Commands::Version => {
            println!("mcp-agent-mail v{}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}
