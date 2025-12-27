#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

MCP_SERVER_NAME="mcp_agent_mail"
MCP_SERVER_PORT="${MCP_AGENT_MAIL_PORT:-8765}"
MCP_SERVER_HOST="${MCP_AGENT_MAIL_HOST:-127.0.0.1}"

CODEX_CONFIG_DIR=".codex"
CODEX_CONFIG_FILE="$CODEX_CONFIG_DIR/config.toml"

log_info() { echo -e "${BLUE}ℹ${NC} $1"; }
log_success() { echo -e "${GREEN}✓${NC} $1"; }
log_warn() { echo -e "${YELLOW}⚠${NC} $1"; }
log_error() { echo -e "${RED}✗${NC} $1"; }

print_header() {
    echo ""
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}     MCP Agent Mail - OpenAI Codex CLI Integration          ${BLUE}║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

detect_codex() {
    log_info "Detecting Codex CLI..."

    if command -v codex &> /dev/null; then
        local version
        version=$(codex --version 2>/dev/null || echo "unknown")
        log_success "Found Codex CLI: $version"
        return 0
    fi

    log_warn "Codex CLI not detected"
    log_info "Install from: npm install -g @openai/codex"
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

update_codex_config() {
    local project_dir="${1:-.}"
    local config_file="$project_dir/$CODEX_CONFIG_FILE"

    log_info "Updating Codex config: $config_file"

    mkdir -p "$project_dir/$CODEX_CONFIG_DIR"

    if [[ -f "$config_file" ]]; then
        cp "$config_file" "${config_file}.backup.$(date +%Y%m%d%H%M%S)"
        log_info "Created backup of existing config"
    fi

    cat > "$config_file" << EOF
[mcp_servers.$MCP_SERVER_NAME]
transport = "http"
url = "http://$MCP_SERVER_HOST:$MCP_SERVER_PORT/mcp/"
EOF

    log_success "Updated $config_file"
}

verify_installation() {
    local project_dir="${1:-.}"
    local config_file="$project_dir/$CODEX_CONFIG_FILE"

    log_info "Verifying installation..."

    if [[ -f "$config_file" ]]; then
        if grep -q "$MCP_SERVER_NAME" "$config_file" 2>/dev/null; then
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
    echo "  • Config: $CODEX_CONFIG_FILE"
    echo ""
    echo "Next steps:"
    echo "  1. Ensure MCP Agent Mail server is running:"
    echo "     am serve http --port $MCP_SERVER_PORT"
    echo "  2. Run Codex in the project directory"
    echo ""
}

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Configure OpenAI Codex CLI to use MCP Agent Mail via HTTP transport.

Codex uses TOML config in .codex/config.toml (project-local).

Options:
  -p, --project DIR     Project directory (default: current)
  -h, --help            Show this help message

Examples:
  $(basename "$0")                    # Configure in current directory
  $(basename "$0") --project /path    # Configure in specific project
EOF
}

PROJECT_DIR="."

while [[ $# -gt 0 ]]; do
    case $1 in
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
    detect_codex
    find_mcp_server
    update_codex_config "$PROJECT_DIR"
    verify_installation "$PROJECT_DIR"
    print_summary
}

main "$@"
