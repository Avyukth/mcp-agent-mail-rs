use clap::{Args, Parser, Subcommand};
use lib_common::config::AppConfig;
use lib_mcp::{
    docs::generate_markdown_docs,
    tools::get_tool_schemas,
    run_stdio, run_sse,
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

    /// Export JSON schemas for all tools
    Schema {
        /// Output format: json or markdown
        #[arg(short, long, default_value = "json")]
        format: String,
        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// List all available tools
    Tools,

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
        tracing::warn!("Failed to load config file: {}. Using defaults.", e);
        AppConfig::default()
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
                    "sse" => run_sse(config.mcp).await?,
                    _ => run_stdio(config.mcp).await?,
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
        Commands::Schema { format, output } => {
            let schemas = get_tool_schemas();
            let content = match format.as_str() {
                "markdown" | "md" => generate_markdown_docs(&schemas),
                _ => serde_json::to_string_pretty(&schemas)?,
            };
            if let Some(path) = output {
                std::fs::write(&path, &content)?;
                eprintln!("Schema written to {}", path);
            } else {
                println!("{}", content);
            }
        }
        Commands::Tools => {
            let schemas = get_tool_schemas();
            println!("MCP Agent Mail Tools ({} total)\n", schemas.len());
            println!("{:<30} DESCRIPTION", "TOOL");
            println!("{}", "-".repeat(80));
            for schema in schemas {
                println!("{:<30} {}", schema.name, schema.description);
            }
        }
        Commands::Version => {
            println!("mcp-agent-mail v{}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}
