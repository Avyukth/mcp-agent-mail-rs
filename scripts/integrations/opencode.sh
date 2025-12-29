#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

MCP_SERVER_NAME="mouchak-mail"
OPENCODE_CONFIG_DIR="$HOME/.config/opencode"
OPENCODE_CONFIG_FILE="$OPENCODE_CONFIG_DIR/config.json"
OPENCODE_MCP_CONFIG=".opencode/mcp.json"

log_info() { echo -e "${BLUE}ℹ${NC} $1"; }
log_success() { echo -e "${GREEN}✓${NC} $1"; }
log_warn() { echo -e "${YELLOW}⚠${NC} $1"; }
log_error() { echo -e "${RED}✗${NC} $1"; }

print_header() {
    echo ""
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}     Mouchak Mail - OpenCode Integration                  ${BLUE}║${NC}"
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

detect_opencode() {
    log_info "Detecting OpenCode..."

    if command -v opencode &> /dev/null; then
        local version
        version=$(opencode --version 2>/dev/null || echo "unknown")
        log_success "Found OpenCode: $version"
        return 0
    fi

    if [[ -d "$OPENCODE_CONFIG_DIR" ]]; then
        log_success "Found OpenCode config directory"
        return 0
    fi

    log_warn "OpenCode not detected in PATH"
    log_info "Install from: https://github.com/opencode-ai/opencode"
    return 0
}

find_mcp_server() {
    log_info "Locating Mouchak Mail binary..."

    if [[ -n "${MCP_SERVER_PATH:-}" ]] && [[ -x "$MCP_SERVER_PATH" ]]; then
        log_success "Using provided MCP_SERVER_PATH: $MCP_SERVER_PATH"
        return 0
    fi

    if command -v am &> /dev/null; then
        MCP_SERVER_PATH=$(command -v am)
        log_success "Found 'am' alias: $MCP_SERVER_PATH"
        return 0
    fi

    if command -v mouchak-mail &> /dev/null; then
        MCP_SERVER_PATH=$(command -v mouchak-mail)
        log_success "Found mouchak-mail: $MCP_SERVER_PATH"
        return 0
    fi

    local target_paths=(
        "$PROJECT_ROOT/target/release/mouchak-mail"
        "$PROJECT_ROOT/target/debug/mouchak-mail"
        "$HOME/.local/bin/am"
        "$HOME/.cargo/bin/mouchak-mail"
    )

    for path in "${target_paths[@]}"; do
        if [[ -x "$path" ]]; then
            MCP_SERVER_PATH="$path"
            log_success "Found Mouchak Mail: $MCP_SERVER_PATH"
            return 0
        fi
    done

    log_error "Mouchak Mail binary not found!"
    echo "  Install with: cargo install --path crates/services/mouchak-mail"
    exit 1
}

update_opencode_config() {
    local config_file="${1:-$OPENCODE_MCP_CONFIG}"
    log_info "Updating OpenCode config: $config_file"

    mkdir -p "$(dirname "$config_file")"

    if [[ -f "$config_file" ]]; then
        cp "$config_file" "${config_file}.backup.$(date +%Y%m%d%H%M%S)"
        log_info "Created backup of existing config"
    fi

    local mcp_config
    mcp_config=$(cat <<EOF
{
  "command": "$MCP_SERVER_PATH",
  "args": ["serve", "mcp", "--transport", "stdio"],
  "env": {
    "RUST_LOG": "info"
  }
}
EOF
)

    if [[ -f "$config_file" ]]; then
        local existing
        existing=$(cat "$config_file")
        if echo "$existing" | jq -e '.mcpServers' &> /dev/null; then
            echo "$existing" | jq --argjson config "$mcp_config" \
                ".mcpServers[\"$MCP_SERVER_NAME\"] = \$config" > "$config_file"
        else
            echo "$existing" | jq --argjson config "$mcp_config" \
                ". + {mcpServers: {\"$MCP_SERVER_NAME\": \$config}}" > "$config_file"
        fi
    else
        jq -n --argjson config "$mcp_config" \
            "{mcpServers: {\"$MCP_SERVER_NAME\": \$config}}" > "$config_file"
    fi

    log_success "Updated $config_file"
}

verify_installation() {
    local config_file="${1:-$OPENCODE_MCP_CONFIG}"
    log_info "Verifying installation..."

    if [[ -f "$config_file" ]]; then
        if jq -e ".mcpServers[\"$MCP_SERVER_NAME\"]" "$config_file" &> /dev/null; then
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
    echo "  • Config: $OPENCODE_MCP_CONFIG"
    echo ""
    echo "Next steps:"
    echo "  1. Restart OpenCode to load the new configuration"
    echo "  2. Mouchak Mail tools should now be available"
    echo ""
}

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Configure OpenCode to use Mouchak Mail via STDIO transport.

Options:
  -g, --global          Configure globally (~/.config/opencode/)
  -p, --project DIR     Configure for specific project
  -h, --help            Show this help message

Examples:
  $(basename "$0")                    # Project config (.opencode/mcp.json)
  $(basename "$0") --global           # Global config
EOF
}

SCOPE="project"
PROJECT_DIR="."

while [[ $# -gt 0 ]]; do
    case $1 in
        -g|--global)
            SCOPE="global"
            shift
            ;;
        -p|--project)
            PROJECT_DIR="$2"
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

main() {
    print_header
    check_dependencies
    detect_opencode
    find_mcp_server

    if [[ "$SCOPE" == "global" ]]; then
        update_opencode_config "$OPENCODE_CONFIG_FILE"
        verify_installation "$OPENCODE_CONFIG_FILE"
    else
        update_opencode_config "$PROJECT_DIR/$OPENCODE_MCP_CONFIG"
        verify_installation "$PROJECT_DIR/$OPENCODE_MCP_CONFIG"
    fi

    print_summary
}

main "$@"
