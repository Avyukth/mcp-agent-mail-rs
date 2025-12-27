#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

MCP_SERVER_NAME="mcp-agent-mail"
MCP_SERVER_PORT="${MCP_AGENT_MAIL_PORT:-8765}"
MCP_SERVER_HOST="${MCP_AGENT_MAIL_HOST:-127.0.0.1}"

GEMINI_CONFIG_DIR="$HOME/.gemini"
GEMINI_MCP_CONFIG="$GEMINI_CONFIG_DIR/settings/mcp_settings.json"

log_info() { echo -e "${BLUE}ℹ${NC} $1"; }
log_success() { echo -e "${GREEN}✓${NC} $1"; }
log_warn() { echo -e "${YELLOW}⚠${NC} $1"; }
log_error() { echo -e "${RED}✗${NC} $1"; }

print_header() {
    echo ""
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}     MCP Agent Mail - Gemini CLI Integration                ${BLUE}║${NC}"
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

detect_gemini() {
    log_info "Detecting Gemini CLI..."

    if command -v gemini &> /dev/null; then
        local version
        version=$(gemini --version 2>/dev/null || echo "unknown")
        log_success "Found Gemini CLI: $version"
        return 0
    fi

    if [[ -d "$GEMINI_CONFIG_DIR" ]]; then
        log_success "Found Gemini config directory"
        return 0
    fi

    log_warn "Gemini CLI not detected"
    log_info "Install from: https://ai.google.dev/gemini-api/docs/cli"
    return 0
}

find_mcp_server() {
    log_info "Locating MCP Agent Mail server..."

    if curl -s "http://$MCP_SERVER_HOST:$MCP_SERVER_PORT/api/health" &> /dev/null; then
        log_success "MCP Agent Mail server running on port $MCP_SERVER_PORT"
        return 0
    fi

    log_warn "Server not running. Start with: am serve http --port $MCP_SERVER_PORT"
    return 0
}

update_gemini_config() {
    log_info "Updating Gemini MCP config: $GEMINI_MCP_CONFIG"

    mkdir -p "$(dirname "$GEMINI_MCP_CONFIG")"

    if [[ -f "$GEMINI_MCP_CONFIG" ]]; then
        cp "$GEMINI_MCP_CONFIG" "${GEMINI_MCP_CONFIG}.backup.$(date +%Y%m%d%H%M%S)"
        log_info "Created backup of existing config"
    fi

    local mcp_config
    mcp_config=$(cat <<EOF
{
  "type": "http",
  "url": "http://$MCP_SERVER_HOST:$MCP_SERVER_PORT/mcp/"
}
EOF
)

    if [[ -f "$GEMINI_MCP_CONFIG" ]]; then
        local existing
        existing=$(cat "$GEMINI_MCP_CONFIG")
        if echo "$existing" | jq -e '.mcpServers' &> /dev/null; then
            echo "$existing" | jq --argjson config "$mcp_config" \
                ".mcpServers[\"$MCP_SERVER_NAME\"] = \$config" > "$GEMINI_MCP_CONFIG"
        else
            echo "$existing" | jq --argjson config "$mcp_config" \
                ". + {mcpServers: {\"$MCP_SERVER_NAME\": \$config}}" > "$GEMINI_MCP_CONFIG"
        fi
    else
        jq -n --argjson config "$mcp_config" \
            "{mcpServers: {\"$MCP_SERVER_NAME\": \$config}}" > "$GEMINI_MCP_CONFIG"
    fi

    log_success "Updated $GEMINI_MCP_CONFIG"
}

verify_installation() {
    log_info "Verifying installation..."

    if [[ -f "$GEMINI_MCP_CONFIG" ]]; then
        if jq -e ".mcpServers[\"$MCP_SERVER_NAME\"]" "$GEMINI_MCP_CONFIG" &> /dev/null; then
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
    echo "  • URL: http://$MCP_SERVER_HOST:$MCP_SERVER_PORT/mcp/"
    echo "  • Config: $GEMINI_MCP_CONFIG"
    echo ""
    echo "Next steps:"
    echo "  1. Ensure MCP Agent Mail server is running:"
    echo "     am serve http --port $MCP_SERVER_PORT"
    echo "  2. Run Gemini CLI with MCP support"
    echo ""
}

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Configure Gemini CLI to use MCP Agent Mail via HTTP transport.

Options:
  -h, --help            Show this help message

Examples:
  $(basename "$0")      # Configure Gemini CLI
EOF
}

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

main() {
    print_header
    check_dependencies
    detect_gemini
    find_mcp_server
    update_gemini_config
    verify_installation
    print_summary
}

main "$@"
