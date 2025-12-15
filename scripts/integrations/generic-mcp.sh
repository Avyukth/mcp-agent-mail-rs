#!/usr/bin/env bash
# generic-mcp.sh - Generic MCP client configuration for any MCP-compatible tool
# Part of mcp-agent-mail-rs integration scripts

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

MCP_SERVER_NAME="mcp-agent-mail"
MCP_SERVER_PORT="${MCP_AGENT_MAIL_PORT:-8765}"
MCP_SERVER_HOST="${MCP_AGENT_MAIL_HOST:-127.0.0.1}"
TRANSPORT_MODE="stdio"

log_info() { echo -e "${BLUE}ℹ${NC} $1"; }
log_success() { echo -e "${GREEN}✓${NC} $1"; }
log_warn() { echo -e "${YELLOW}⚠${NC} $1"; }
log_error() { echo -e "${RED}✗${NC} $1"; }

print_header() {
    echo ""
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}     MCP Agent Mail - Generic MCP Client Setup             ${BLUE}║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

check_dependencies() {
    log_info "Checking dependencies..."

    if ! command -v jq &> /dev/null; then
        log_error "jq is required but not installed."
        echo "  Install with: brew install jq (macOS) or apt install jq (Linux)"
        exit 1
    fi

    log_success "Dependencies satisfied"
}

find_mcp_binaries() {
    log_info "Locating MCP server binaries..."

    # Find stdio server
    if command -v mcp-stdio-server &> /dev/null; then
        MCP_STDIO_PATH=$(which mcp-stdio-server)
    else
        local stdio_paths=(
            "$PROJECT_ROOT/target/release/mcp-stdio-server"
            "$PROJECT_ROOT/target/debug/mcp-stdio-server"
        )
        for path in "${stdio_paths[@]}"; do
            if [[ -x "$path" ]]; then
                MCP_STDIO_PATH="$path"
                break
            fi
        done
    fi

    # Find HTTP server
    if command -v mcp-server &> /dev/null; then
        MCP_HTTP_PATH=$(which mcp-server)
    else
        local http_paths=(
            "$PROJECT_ROOT/target/release/mcp-server"
            "$PROJECT_ROOT/target/debug/mcp-server"
        )
        for path in "${http_paths[@]}"; do
            if [[ -x "$path" ]]; then
                MCP_HTTP_PATH="$path"
                break
            fi
        done
    fi

    if [[ -n "${MCP_STDIO_PATH:-}" ]]; then
        log_success "Found mcp-stdio-server: $MCP_STDIO_PATH"
    else
        log_warn "mcp-stdio-server not found"
    fi

    if [[ -n "${MCP_HTTP_PATH:-}" ]]; then
        log_success "Found mcp-server: $MCP_HTTP_PATH"
    else
        log_warn "mcp-server not found"
    fi

    if [[ -z "${MCP_STDIO_PATH:-}" ]] && [[ -z "${MCP_HTTP_PATH:-}" ]]; then
        log_error "No MCP server binaries found!"
        echo "  Build with: cd $PROJECT_ROOT && cargo build --release"
        exit 1
    fi
}

generate_stdio_config() {
    local output_file="$1"

    if [[ -z "${MCP_STDIO_PATH:-}" ]]; then
        log_error "mcp-stdio-server not available for stdio config"
        return 1
    fi

    cat > "$output_file" <<EOF
{
  "mcpServers": {
    "$MCP_SERVER_NAME": {
      "command": "$MCP_STDIO_PATH",
      "args": [],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
EOF

    log_success "Generated stdio config: $output_file"
}

generate_http_config() {
    local output_file="$1"

    if [[ -z "${MCP_HTTP_PATH:-}" ]]; then
        log_error "mcp-server not available for HTTP config"
        return 1
    fi

    cat > "$output_file" <<EOF
{
  "mcpServers": {
    "$MCP_SERVER_NAME": {
      "url": "http://$MCP_SERVER_HOST:$MCP_SERVER_PORT",
      "transport": "http"
    }
  }
}
EOF

    log_success "Generated HTTP config: $output_file"
}

generate_sse_config() {
    local output_file="$1"

    if [[ -z "${MCP_HTTP_PATH:-}" ]]; then
        log_error "mcp-server not available for SSE config"
        return 1
    fi

    cat > "$output_file" <<EOF
{
  "mcpServers": {
    "$MCP_SERVER_NAME": {
      "url": "http://$MCP_SERVER_HOST:$MCP_SERVER_PORT/sse",
      "transport": "sse"
    }
  }
}
EOF

    log_success "Generated SSE config: $output_file"
}

create_config_examples() {
    log_info "Creating configuration examples..."

    local config_dir="mcp-agent-mail-configs"
    mkdir -p "$config_dir"

    # Generate all transport modes
    if [[ -n "${MCP_STDIO_PATH:-}" ]]; then
        generate_stdio_config "$config_dir/stdio-config.json"
    fi

    if [[ -n "${MCP_HTTP_PATH:-}" ]]; then
        generate_http_config "$config_dir/http-config.json"
        generate_sse_config "$config_dir/sse-config.json"
    fi

    # Create README
    cat > "$config_dir/README.md" <<EOF
# MCP Agent Mail Configuration Examples

This directory contains example MCP server configurations for various transport modes.

## Available Configurations

### STDIO Transport (stdio-config.json)
- Direct process communication
- Best for: Claude Desktop, Cursor, Cline, Windsurf
- Server binary: $MCP_STDIO_PATH

### HTTP Transport (http-config.json)
- RESTful HTTP API
- Best for: Custom integrations, testing
- Server binary: $MCP_HTTP_PATH
- Endpoint: http://$MCP_SERVER_HOST:$MCP_SERVER_PORT

### SSE Transport (sse-config.json)
- Server-Sent Events for streaming
- Best for: Web applications, real-time updates
- Server binary: $MCP_HTTP_PATH
- Endpoint: http://$MCP_SERVER_HOST:$MCP_SERVER_PORT/sse

## Usage

### For MCP Clients with Native Support

Copy the appropriate config to your client's configuration file:

#### Claude Desktop
\`\`\`bash
# macOS
cp stdio-config.json ~/Library/Application\\ Support/Claude/claude_desktop_config.json

# Linux
cp stdio-config.json ~/.config/Claude/claude_desktop_config.json
\`\`\`

#### Cursor
\`\`\`bash
cp stdio-config.json ~/.cursor/mcp_settings.json
\`\`\`

#### Generic MCP Client
Merge the config into your client's settings file.

### Manual Integration

1. Copy the config content
2. Add to your MCP client's configuration
3. Restart the client
4. Start the MCP server if using HTTP/SSE

### Starting the Server

#### STDIO Mode
Server is started automatically by the MCP client.

#### HTTP/SSE Mode
\`\`\`bash
$MCP_HTTP_PATH
\`\`\`

## Environment Variables

- \`MCP_AGENT_MAIL_PORT\` - Server port (default: $MCP_SERVER_PORT)
- \`MCP_AGENT_MAIL_HOST\` - Server host (default: $MCP_SERVER_HOST)
- \`RUST_LOG\` - Log level (debug, info, warn, error)

## Available MCP Tools

Once configured, your MCP client will have access to 28 tools:

### Agent Management
- \`agent_register\` - Register as an agent
- \`agent_whois\` - Look up agent information
- \`agent_list\` - List all agents
- \`agent_update_profile\` - Update agent profile

### Messaging
- \`message_send\` - Send a message
- \`message_reply\` - Reply to a message
- \`inbox_list\` - List inbox messages
- \`message_search\` - Search messages
- \`message_get\` - Get message details
- \`message_mark_read\` - Mark message as read

### File Reservations
- \`file_reservation_paths\` - Reserve file paths
- \`file_reservation_list\` - List active reservations
- \`file_reservation_release\` - Release reservations
- \`file_reservation_renew\` - Extend reservation TTL

### Project Management
- \`project_ensure\` - Create or get project
- \`project_list\` - List all projects
- \`project_init_git\` - Initialize git archive

...and 11 more tools!

## Testing the Integration

\`\`\`bash
# Check server health
curl http://$MCP_SERVER_HOST:$MCP_SERVER_PORT/health

# List available tools (if using mcp-cli)
cargo run -p mcp-cli -- tools
\`\`\`

## Troubleshooting

### Server Not Starting
- Check if port $MCP_SERVER_PORT is available
- Verify binary exists and is executable
- Check logs with: RUST_LOG=debug

### Client Not Finding Server
- Ensure config file is in the correct location
- Restart the client application
- Check client logs for MCP errors

### Permission Errors
- Make sure binaries are executable: \`chmod +x path/to/binary\`
- Check file permissions on config files

## More Information

- Project: $PROJECT_ROOT
- Documentation: $PROJECT_ROOT/docs/
- API Reference: $PROJECT_ROOT/README.md
EOF

    log_success "Created configuration examples in: $config_dir/"
}

print_summary() {
    echo ""
    echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║${NC}     Configuration Examples Created!                         ${GREEN}║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo "Configuration files created in: mcp-agent-mail-configs/"
    echo ""

    if [[ -n "${MCP_STDIO_PATH:-}" ]]; then
        echo "  • stdio-config.json - For STDIO transport"
    fi
    if [[ -n "${MCP_HTTP_PATH:-}" ]]; then
        echo "  • http-config.json - For HTTP REST API"
        echo "  • sse-config.json - For Server-Sent Events"
    fi
    echo "  • README.md - Usage instructions"
    echo ""

    echo "Quick start for common clients:"
    echo ""
    echo "Claude Desktop (macOS):"
    echo "  cp mcp-agent-mail-configs/stdio-config.json \\"
    echo "     ~/Library/Application\\ Support/Claude/claude_desktop_config.json"
    echo ""
    echo "Cursor:"
    echo "  cp mcp-agent-mail-configs/stdio-config.json ~/.cursor/mcp_settings.json"
    echo ""
    echo "Custom client:"
    echo "  See mcp-agent-mail-configs/README.md for detailed instructions"
    echo ""
    echo "Test the server:"
    if [[ -n "${MCP_HTTP_PATH:-}" ]]; then
        echo "  $MCP_HTTP_PATH &"
        echo "  curl http://$MCP_SERVER_HOST:$MCP_SERVER_PORT/health"
    fi
    echo ""
}

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Generate MCP client configuration examples for any MCP-compatible tool.

This script creates example configuration files for different MCP transport
modes (stdio, HTTP, SSE) that can be used with any MCP client.

Options:
  -p, --port PORT       MCP server port (default: 8765)
  -H, --host HOST       MCP server host (default: 127.0.0.1)
  -h, --help            Show this help message

Examples:
  $(basename "$0")                           # Generate all configs
  $(basename "$0") --port 9000               # Use custom port
  $(basename "$0") --host 0.0.0.0            # Bind to all interfaces

Output:
  Creates mcp-agent-mail-configs/ directory with:
  - stdio-config.json (if mcp-stdio-server available)
  - http-config.json (if mcp-server available)
  - sse-config.json (if mcp-server available)
  - README.md (usage instructions)

EOF
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -p|--port)
            MCP_SERVER_PORT="$2"
            shift 2
            ;;
        -H|--host)
            MCP_SERVER_HOST="$2"
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            usage
            exit 1
            ;;
    esac
done

# Main execution
main() {
    print_header
    check_dependencies
    find_mcp_binaries
    create_config_examples
    print_summary
}

main "$@"
