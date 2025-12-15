use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use lib_mcp::{run_stdio, run_sse, tools::get_tool_schemas};
use lib_common::config::McpConfig;

#[derive(Parser)]
#[command(name = "mcp-agent-mail")]
#[command(about = "MCP Agent Mail - Multi-agent messaging system")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the MCP server over stdio (default)
    Serve {
        /// Transport mode: stdio or sse
        #[arg(short, long, default_value = "stdio")]
        transport: String,
        /// Port for SSE server (default: 3000)
        #[arg(short, long, default_value = "3000")]
        port: u16,
        /// Host to bind SSE server (default: 127.0.0.1)
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
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
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Note: Logging setup is tricky here. 
    // lib-mcp doesn't set up logging, it assumes caller does.
    // mcp-stdio used to set up stderr logging.
    // We should replicate that here for the standalone binary.
    
    match cli.command.unwrap_or(Commands::Serve {
        transport: "stdio".to_string(),
        port: 3000,
        host: "127.0.0.1".to_string(),
    }) {
        Commands::Serve { transport, port, host: _ } => {
            // Setup logging for standalone mode
            tracing_subscriber::registry()
                .with(fmt::layer().with_writer(std::io::stderr))
                .with(EnvFilter::from_default_env().add_directive("mcp_stdio=info".parse()?))
                .init();

            let config = McpConfig {
                transport: transport.clone(),
                port,
            };

            match transport.as_str() {
                "sse" => run_sse(config).await,
                _ => run_stdio(config).await,
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
            Ok(())
        },
        Commands::Tools => {
            let schemas = get_tool_schemas();
            println!("MCP Agent Mail Tools ({} total)\n", schemas.len());
            println!("{:<30} DESCRIPTION", "TOOL");
            println!("{}", "-".repeat(80));
            for schema in schemas {
                println!("{:<30} {}", schema.name, schema.description);
            }
            Ok(())
        },
    }
}

// Helper needed because it was local in main.rs before
fn generate_markdown_docs(schemas: &[lib_mcp::tools::ToolSchema]) -> String {
    let mut md = String::from("# MCP Agent Mail - Tool Reference\n\n");
    md.push_str(&format!("Total tools: {}\n\n", schemas.len()));
    md.push_str("## Table of Contents\n\n");

    for schema in schemas {
        md.push_str(&format!("- [{}](#{})\n", schema.name, schema.name.replace('_', "-")));
    }

    md.push_str("\n---\n\n");

    for schema in schemas {
        md.push_str(&format!("## {}\n\n", schema.name));
        md.push_str(&format!("{}\n\n", schema.description));

        if !schema.parameters.is_empty() {
            md.push_str("### Parameters\n\n");
            md.push_str("| Name | Type | Required | Description |\n");
            md.push_str("|------|------|----------|-------------|\n");
            md.push_str("|------|------|----------|-------------|\n"); 
            // Loop params logic needs to be replicated or access schema internals
            // ToolSchema in lib-mcp needs to be public inspectable. 
            // Assuming it is standard struct.
             for param in &schema.parameters {
                md.push_str(&format!(
                    "| `{}` | {} | {} | {} |\n",
                    param.name,
                    param.param_type,
                    if param.required { "Yes" } else { "No" },
                    param.description
                ));
            }
        }
    }
    md
}
