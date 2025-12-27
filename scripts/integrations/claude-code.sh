#!/usr/bin/env bash
# claude-code.sh - Configure Claude Code CLI to use MCP Agent Mail
# Part of mcp-agent-mail-rs integration scripts
#
# Claude Code config locations (by scope):
#   • User config: ~/.claude.json
#   • Project config (shared via .mcp.json): <project>/.mcp.json
#   • Local config: ~/.claude.json [project: <path>]

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Default configuration
MCP_SERVER_PORT="${MCP_AGENT_MAIL_PORT:-8765}"
MCP_SERVER_HOST="${MCP_AGENT_MAIL_HOST:-127.0.0.1}"
MCP_SERVER_NAME="mcp-agent-mail"

# Config file locations (NEW Claude Code format)
CLAUDE_CONFIG_USER="$HOME/.claude.json"
CLAUDE_CONFIG_PROJECT=".mcp.json"

log_info() { echo -e "${BLUE}ℹ${NC} $1"; }
log_success() { echo -e "${GREEN}✓${NC} $1"; }
log_warn() { echo -e "${YELLOW}⚠${NC} $1"; }
log_error() { echo -e "${RED}✗${NC} $1"; }

print_header() {
    echo ""
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}     MCP Agent Mail - Claude Code Integration               ${BLUE}║${NC}"
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

detect_claude_code() {
    log_info "Detecting Claude Code CLI..."

    if command -v claude &> /dev/null; then
        CLAUDE_VERSION=$(claude --version 2>/dev/null || echo "unknown")
        log_success "Found Claude Code: $CLAUDE_VERSION"
        return 0
    fi

    log_warn "Claude Code CLI not found in PATH"
    log_info "Install from: https://claude.ai/code"
    return 0
}

find_mcp_server() {
    log_info "Locating MCP Agent Mail binary..."

    # Check if already set via environment
    if [[ -n "${MCP_SERVER_PATH:-}" ]] && [[ -x "$MCP_SERVER_PATH" ]]; then
        log_success "Using provided MCP_SERVER_PATH: $MCP_SERVER_PATH"
        return 0
    fi

    # Check for 'am' alias first (recommended)
    if command -v am &> /dev/null; then
        MCP_SERVER_PATH=$(command -v am)
        log_success "Found 'am' alias: $MCP_SERVER_PATH"
        return 0
    fi

    # Check for full binary name
    if command -v mcp-agent-mail &> /dev/null; then
        MCP_SERVER_PATH=$(command -v mcp-agent-mail)
        log_success "Found mcp-agent-mail: $MCP_SERVER_PATH"
        return 0
    fi

    # Check project build directories
    local target_paths=(
        "$PROJECT_ROOT/target/release/mcp-agent-mail"
        "$PROJECT_ROOT/target/debug/mcp-agent-mail"
        "$HOME/.local/bin/am"
        "$HOME/.cargo/bin/mcp-agent-mail"
    )

    for path in "${target_paths[@]}"; do
        if [[ -x "$path" ]]; then
            MCP_SERVER_PATH="$path"
            log_success "Found MCP Agent Mail: $MCP_SERVER_PATH"
            return 0
        fi
    done

    log_error "MCP Agent Mail binary not found!"
    echo "  Build with: cd $PROJECT_ROOT && cargo build --release -p mcp-agent-mail"
    echo "  Or install: cargo install --path crates/services/mcp-agent-mail"
    exit 1
}

generate_mcp_config() {
    local mode="${1:-stdio}"

    if [[ "$mode" == "stdio" ]]; then
        cat <<EOF
{
  "command": "$MCP_SERVER_PATH",
  "args": ["serve", "mcp", "--transport", "stdio"],
  "env": {
    "RUST_LOG": "info"
  }
}
EOF
    else
        # SSE mode
        cat <<EOF
{
  "type": "sse",
  "url": "http://$MCP_SERVER_HOST:$MCP_SERVER_PORT/sse"
}
EOF
    fi
}

update_claude_config() {
    local config_file="$1"
    local scope="$2"
    local mode="${3:-stdio}"

    log_info "Updating $scope config: $config_file"

    # Create directory if needed
    mkdir -p "$(dirname "$config_file")"

    # Create backup if file exists
    if [[ -f "$config_file" ]]; then
        cp "$config_file" "${config_file}.backup.$(date +%Y%m%d%H%M%S)"
        log_info "Created backup of existing config"
    fi

    # Generate MCP server config
    local mcp_config
    mcp_config=$(generate_mcp_config "$mode")

    # Create or update config file
    if [[ -f "$config_file" ]]; then
        # File exists - merge configuration
        local existing
        existing=$(cat "$config_file")

        # Check if mcpServers key exists
        if echo "$existing" | jq -e '.mcpServers' &> /dev/null; then
            # Add/update our server entry
            echo "$existing" | jq --argjson config "$mcp_config" \
                ".mcpServers[\"$MCP_SERVER_NAME\"] = \$config" > "$config_file"
        else
            # Add mcpServers object
            echo "$existing" | jq --argjson config "$mcp_config" \
                ". + {mcpServers: {\"$MCP_SERVER_NAME\": \$config}}" > "$config_file"
        fi
    else
        # Create new config file
        jq -n --argjson config "$mcp_config" \
            "{mcpServers: {\"$MCP_SERVER_NAME\": \$config}}" > "$config_file"
    fi

    log_success "Updated $config_file"
}

setup_user_scope() {
    log_info "Setting up user-scope integration..."
    update_claude_config "$CLAUDE_CONFIG_USER" "user" "$MODE"
}

setup_project_scope() {
    local project_dir="${1:-.}"
    local config_file="$project_dir/$CLAUDE_CONFIG_PROJECT"

    log_info "Setting up project-scope integration in $project_dir..."
    update_claude_config "$config_file" "project" "$MODE"
}

verify_installation() {
    log_info "Verifying installation..."

    if [[ -f "$CLAUDE_CONFIG_USER" ]]; then
        if jq -e ".mcpServers[\"$MCP_SERVER_NAME\"]" "$CLAUDE_CONFIG_USER" &> /dev/null; then
            log_success "User config verified: $CLAUDE_CONFIG_USER"
        else
            log_warn "MCP server not found in user config"
        fi
    fi

    if [[ "$SCOPE" == "project" ]] && [[ -f "$PROJECT_DIR/$CLAUDE_CONFIG_PROJECT" ]]; then
        if jq -e ".mcpServers[\"$MCP_SERVER_NAME\"]" "$PROJECT_DIR/$CLAUDE_CONFIG_PROJECT" &> /dev/null; then
            log_success "Project config verified: $PROJECT_DIR/$CLAUDE_CONFIG_PROJECT"
        fi
    fi
}

print_summary() {
    echo ""
    echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║${NC}     Integration Complete!                                   ${GREEN}║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo "Configuration:"
    echo "  • Server: $MCP_SERVER_NAME"
    echo "  • Binary: $MCP_SERVER_PATH"
    echo "  • Mode: $MODE"
    echo ""
    echo "Config files updated:"
    if [[ "$SCOPE" == "user" ]]; then
        echo "  • $CLAUDE_CONFIG_USER (user scope)"
    else
        echo "  • $PROJECT_DIR/$CLAUDE_CONFIG_PROJECT (project scope)"
    fi
    echo ""
    echo "Next steps:"
    echo "  1. Restart Claude Code to load the new configuration"
    echo "  2. MCP Agent Mail tools should now be available"
    echo "  3. Use '/mcp' in Claude Code to check server status"
    echo ""
    echo "Available MCP tools include:"
    echo "  • register_agent - Register a new agent"
    echo "  • send_message - Send messages to other agents"
    echo "  • check_inbox - Check your inbox"
    echo "  • file_reservation_paths - Reserve files for editing"
    echo "  • ...and 40+ more tools!"
    echo ""
}

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Configure Claude Code CLI to use MCP Agent Mail via STDIO transport.

Claude Code config locations:
  • User config: ~/.claude.json
  • Project config: .mcp.json (shared via git)

Options:
  -m, --mode MODE       Connection mode: stdio (default) or sse
  -s, --scope SCOPE     Config scope: user (default) or project
  -p, --project DIR     Project directory for project-scope config
  -h, --help            Show this help message

Examples:
  $(basename "$0")                           # User scope, STDIO mode
  $(basename "$0") --mode sse                # User scope, SSE mode
  $(basename "$0") --scope project           # Project scope (.mcp.json)
  $(basename "$0") --project /path/to/proj   # Project scope in specific dir

Environment Variables:
  MCP_AGENT_MAIL_PORT   Server port for SSE mode (default: 8765)
  MCP_AGENT_MAIL_HOST   Server host for SSE mode (default: 127.0.0.1)
  MCP_SERVER_PATH       Override MCP binary path

EOF
}

# Parse arguments
MODE="stdio"
SCOPE="user"
PROJECT_DIR="."

while [[ $# -gt 0 ]]; do
    case $1 in
        -m|--mode)
            MODE="$2"
            shift 2
            ;;
        -s|--scope)
            SCOPE="$2"
            shift 2
            ;;
        -p|--project)
            PROJECT_DIR="$2"
            SCOPE="project"
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

# Validate mode
if [[ "$MODE" != "stdio" && "$MODE" != "sse" ]]; then
    log_error "Invalid mode: $MODE (must be 'stdio' or 'sse')"
    exit 1
fi

# Main execution
main() {
    print_header
    check_dependencies
    detect_claude_code
    find_mcp_server

    if [[ "$SCOPE" == "project" ]]; then
        setup_project_scope "$PROJECT_DIR"
    else
        setup_user_scope
    fi

    verify_installation
    print_summary
}

main "$@"
