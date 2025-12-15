#!/usr/bin/env bash
# cursor.sh - Configure Cursor IDE to use MCP Agent Mail
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
CURSOR_CONFIG_DIR="$HOME/.cursor"
CURSOR_MCP_CONFIG="$CURSOR_CONFIG_DIR/mcp_settings.json"

log_info() { echo -e "${BLUE}ℹ${NC} $1"; }
log_success() { echo -e "${GREEN}✓${NC} $1"; }
log_warn() { echo -e "${YELLOW}⚠${NC} $1"; }
log_error() { echo -e "${RED}✗${NC} $1"; }

print_header() {
    echo ""
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}     MCP Agent Mail - Cursor IDE Integration               ${BLUE}║${NC}"
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

detect_cursor() {
    log_info "Detecting Cursor IDE..."

    # Check common Cursor paths
    local cursor_paths=(
        "/Applications/Cursor.app"
        "$HOME/Applications/Cursor.app"
        "/usr/share/applications/cursor.desktop"
    )

    for path in "${cursor_paths[@]}"; do
        if [[ -e "$path" ]]; then
            log_success "Found Cursor at: $path"
            return 0
        fi
    done

    # Check if cursor command exists
    if command -v cursor &> /dev/null; then
        log_success "Found Cursor CLI in PATH"
        return 0
    fi

    log_warn "Cursor IDE not detected"
    log_info "Download from: https://cursor.sh"
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

update_cursor_config() {
    log_info "Updating Cursor MCP settings: $CURSOR_MCP_CONFIG"

    # Create directory if needed
    mkdir -p "$CURSOR_CONFIG_DIR"

    # Create backup if file exists
    if [[ -f "$CURSOR_MCP_CONFIG" ]]; then
        cp "$CURSOR_MCP_CONFIG" "${CURSOR_MCP_CONFIG}.backup.$(date +%Y%m%d%H%M%S)"
        log_info "Created backup of existing config"
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

    # Create or update config file
    if [[ -f "$CURSOR_MCP_CONFIG" ]]; then
        # File exists - merge configuration
        local existing
        existing=$(cat "$CURSOR_MCP_CONFIG")

        if echo "$existing" | jq -e '.mcpServers' &> /dev/null; then
            echo "$existing" | jq --argjson config "$mcp_config" \
                ".mcpServers[\"$MCP_SERVER_NAME\"] = \$config" > "$CURSOR_MCP_CONFIG"
        else
            echo "$existing" | jq --argjson config "$mcp_config" \
                ". + {mcpServers: {\"$MCP_SERVER_NAME\": \$config}}" > "$CURSOR_MCP_CONFIG"
        fi
    else
        # Create new config file
        jq -n --argjson config "$mcp_config" \
            "{mcpServers: {\"$MCP_SERVER_NAME\": \$config}}" > "$CURSOR_MCP_CONFIG"
    fi

    log_success "Updated $CURSOR_MCP_CONFIG"
}

verify_installation() {
    log_info "Verifying installation..."

    if [[ -f "$CURSOR_MCP_CONFIG" ]]; then
        if jq -e ".mcpServers[\"$MCP_SERVER_NAME\"]" "$CURSOR_MCP_CONFIG" &> /dev/null; then
            log_success "Configuration verified"
        else
            log_warn "MCP server not found in config"
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
    echo "  • Config: $CURSOR_MCP_CONFIG"
    echo ""
    echo "Next steps:"
    echo "  1. Restart Cursor IDE to load the new configuration"
    echo "  2. Open Cursor's Composer (Cmd+I / Ctrl+I)"
    echo "  3. MCP Agent Mail tools will be available in the AI context"
    echo "  4. Try asking: 'Register me as an agent in this project'"
    echo ""
    echo "Key capabilities:"
    echo "  • Coordinate with other AI agents working on the same codebase"
    echo "  • Reserve files before editing to prevent conflicts"
    echo "  • Send/receive messages about work in progress"
    echo "  • Search archived communication history"
    echo ""
}

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Configure Cursor IDE to use MCP Agent Mail via stdio transport.

Options:
  -h, --help            Show this help message

Examples:
  $(basename "$0")                    # Install for Cursor IDE

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
    detect_cursor
    find_mcp_server
    update_cursor_config
    verify_installation
    print_summary
}

main "$@"
