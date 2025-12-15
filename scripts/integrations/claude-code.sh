#!/usr/bin/env bash
# claude-code.sh - Configure Claude Code CLI to use MCP Agent Mail
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
CLAUDE_SETTINGS="$HOME/.claude/settings.json"

log_info() { echo -e "${BLUE}ℹ${NC} $1"; }
log_success() { echo -e "${GREEN}✓${NC} $1"; }
log_warn() { echo -e "${YELLOW}⚠${NC} $1"; }
log_error() { echo -e "${RED}✗${NC} $1"; }

print_header() {
    echo ""
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}     MCP Agent Mail - Claude Code CLI Integration          ${BLUE}║${NC}"
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
    log_info "Install from: https://www.anthropic.com/claude-code"
    return 0
}

find_mcp_server() {
    log_info "Locating mcp-stdio-server binary..."

    # Check if in PATH
    if command -v mcp-stdio-server &> /dev/null; then
        MCP_SERVER_PATH=$(which mcp-stdio-server)
        log_success "Found mcp-stdio-server: $MCP_SERVER_PATH"
        return 0
    fi

    # Check project build directories
    local target_paths=(
        "$PROJECT_ROOT/target/release/mcp-stdio-server"
        "$PROJECT_ROOT/target/debug/mcp-stdio-server"
    )

    for path in "${target_paths[@]}"; do
        if [[ -x "$path" ]]; then
            MCP_SERVER_PATH="$path"
            log_success "Found mcp-stdio-server: $MCP_SERVER_PATH"
            return 0
        fi
    done

    log_error "mcp-stdio-server not found!"
    echo "  Build it with: cd $PROJECT_ROOT && cargo build --release -p mcp-stdio"
    exit 1
}

update_claude_settings() {
    log_info "Updating Claude Code settings: $CLAUDE_SETTINGS"

    # Create directory if needed
    mkdir -p "$(dirname "$CLAUDE_SETTINGS")"

    # Create backup if file exists
    if [[ -f "$CLAUDE_SETTINGS" ]]; then
        cp "$CLAUDE_SETTINGS" "${CLAUDE_SETTINGS}.backup.$(date +%Y%m%d%H%M%S)"
        log_info "Created backup of existing settings"
    fi

    # Generate MCP server config for stdio transport
    local mcp_config
    mcp_config=$(cat <<EOF
{
  "command": "$MCP_SERVER_PATH",
  "args": [],
  "env": {
    "RUST_LOG": "info"
  }
}
EOF
)

    # Create or update settings file
    if [[ -f "$CLAUDE_SETTINGS" ]]; then
        # File exists - merge configuration
        local existing
        existing=$(cat "$CLAUDE_SETTINGS")

        # Check if mcpServers key exists
        if echo "$existing" | jq -e '.mcpServers' &> /dev/null; then
            # Add/update our server entry
            echo "$existing" | jq --argjson config "$mcp_config" \
                ".mcpServers[\"$MCP_SERVER_NAME\"] = \$config" > "$CLAUDE_SETTINGS"
        else
            # Add mcpServers object
            echo "$existing" | jq --argjson config "$mcp_config" \
                ". + {mcpServers: {\"$MCP_SERVER_NAME\": \$config}}" > "$CLAUDE_SETTINGS"
        fi
    else
        # Create new settings file
        jq -n --argjson config "$mcp_config" \
            "{mcpServers: {\"$MCP_SERVER_NAME\": \$config}}" > "$CLAUDE_SETTINGS"
    fi

    log_success "Updated $CLAUDE_SETTINGS"
}

verify_installation() {
    log_info "Verifying installation..."

    if [[ -f "$CLAUDE_SETTINGS" ]]; then
        if jq -e ".mcpServers[\"$MCP_SERVER_NAME\"]" "$CLAUDE_SETTINGS" &> /dev/null; then
            log_success "Configuration verified"
        else
            log_warn "MCP server not found in settings"
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
    echo "  • Settings: $CLAUDE_SETTINGS"
    echo ""
    echo "Next steps:"
    echo "  1. Restart Claude Code CLI to load the new configuration"
    echo "  2. The MCP Agent Mail tools should now be available"
    echo "  3. Try: claude 'List my MCP servers'"
    echo ""
    echo "Available MCP tools include:"
    echo "  • agent_register - Register a new agent"
    echo "  • message_send - Send messages to other agents"
    echo "  • inbox_list - Check your inbox"
    echo "  • file_reservation_paths - Reserve files for editing"
    echo "  • ...and 24 more tools!"
    echo ""
}

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Configure Claude Code CLI to use MCP Agent Mail via stdio transport.

Options:
  -h, --help            Show this help message

Environment Variables:
  None required - uses stdio transport directly

Examples:
  $(basename "$0")                    # Install for Claude Code CLI

EOF
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
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
    detect_claude_code
    find_mcp_server
    update_claude_settings
    verify_installation
    print_summary
}

main "$@"
