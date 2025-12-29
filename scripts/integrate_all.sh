#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INTEGRATIONS_DIR="$SCRIPT_DIR/integrations"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

MCP_SERVER_PORT="${MOUCHAK_MAIL_PORT:-8765}"
MCP_SERVER_HOST="${MOUCHAK_MAIL_HOST:-127.0.0.1}"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

declare -a DETECTED=()
declare -a CONFIGURED=()
declare -a FAILED=()

MCP_SERVER_PATH=""

log_info() { echo -e "${BLUE}ℹ${NC} $1"; }
log_success() { echo -e "${GREEN}✓${NC} $1"; }
log_warn() { echo -e "${YELLOW}⚠${NC} $1"; }
log_error() { echo -e "${RED}✗${NC} $1"; }

print_header() {
    echo ""
    echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║${NC}  Mouchak Mail - Auto-Detect & Configure All Agents      ${CYAN}║${NC}"
    echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
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
    echo ""
}

find_mcp_server() {
    log_info "Locating Mouchak Mail binary..."

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
    return 1
}

ensure_server_running() {
    log_info "Checking if Mouchak Mail server is running..."

    if curl -s "http://$MCP_SERVER_HOST:$MCP_SERVER_PORT/api/health" &> /dev/null; then
        log_success "Server is running on port $MCP_SERVER_PORT"
        return 0
    fi

    log_warn "Server not running on port $MCP_SERVER_PORT"

    if [[ -z "$MCP_SERVER_PATH" ]]; then
        log_error "Cannot start server - binary not found"
        return 1
    fi

    log_info "Starting Mouchak Mail server..."
    "$MCP_SERVER_PATH" serve http --port "$MCP_SERVER_PORT" &
    local server_pid=$!
    sleep 2

    if curl -s "http://$MCP_SERVER_HOST:$MCP_SERVER_PORT/api/health" &> /dev/null; then
        log_success "Server started successfully (PID: $server_pid)"
        return 0
    else
        log_error "Failed to start server"
        return 1
    fi
}

detect_claude_code() {
    command -v claude &> /dev/null || [[ -f "$HOME/.claude.json" ]]
}

detect_cursor() {
    [[ -d "/Applications/Cursor.app" ]] || [[ -d "$HOME/Applications/Cursor.app" ]] || [[ -d "$HOME/.cursor" ]]
}

detect_windsurf() {
    [[ -d "/Applications/Windsurf.app" ]] || [[ -d "$HOME/.codeium/windsurf" ]]
}

detect_cline() {
    for path in "$HOME/.vscode/extensions/saoudrizwan.claude-dev-"*; do
        [[ -d "$path" ]] && return 0
    done
    return 1
}

detect_copilot() {
    for path in "$HOME/.vscode/extensions/github.copilot-"*; do
        [[ -d "$path" ]] && return 0
    done
    return 1
}

detect_continue() {
    for path in "$HOME/.vscode/extensions/continue.continue-"*; do
        [[ -d "$path" ]] && return 0
    done
    [[ -d "$HOME/.continue" ]] && return 0
    return 1
}

detect_aider() {
    command -v aider &> /dev/null || [[ -f "$HOME/.aider.conf.yml" ]]
}

detect_opencode() {
    command -v opencode &> /dev/null || [[ -d "$HOME/.config/opencode" ]]
}

detect_antigravity() {
    command -v antigravity &> /dev/null || command -v ag &> /dev/null || [[ -d "$HOME/.antigravity" ]]
}

detect_gemini() {
    command -v gemini &> /dev/null || [[ -d "$HOME/.gemini" ]]
}

detect_codex() {
    command -v codex &> /dev/null || [[ -d ".codex" ]]
}

run_detection() {
    log_info "Scanning for installed coding agents..."
    echo ""

    local agents=(
        "claude_code:Claude Code:claude-code.sh"
        "cursor:Cursor IDE:cursor.sh"
        "windsurf:Windsurf IDE:windsurf.sh"
        "cline:Cline (VSCode):cline.sh"
        "copilot:GitHub Copilot:copilot.sh"
        "continue:Continue (VSCode):continue.sh"
        "aider:Aider:aider.sh"
        "opencode:OpenCode:opencode.sh"
        "antigravity:Antigravity:antigravity.sh"
        "gemini:Gemini CLI:gemini.sh"
        "codex:OpenAI Codex:codex.sh"
    )

    for agent_info in "${agents[@]}"; do
        IFS=':' read -r key name script <<< "$agent_info"

        if "detect_$key" 2>/dev/null; then
            echo -e "  ${GREEN}✓${NC} $name detected"
            DETECTED+=("$key:$name:$script")
        else
            echo -e "  ${YELLOW}○${NC} $name not found"
        fi
    done

    echo ""

    if [[ ${#DETECTED[@]} -eq 0 ]]; then
        log_warn "No coding agents detected"
        echo "  Run individual scripts from: $INTEGRATIONS_DIR/"
        return 1
    fi

    log_success "Detected ${#DETECTED[@]} coding agent(s)"
    return 0
}

run_configuration() {
    echo ""
    log_info "Configuring detected agents..."
    echo ""

    export MCP_SERVER_PATH
    export MCP_SERVER_PORT
    export MCP_SERVER_HOST

    for agent_info in "${DETECTED[@]}"; do
        IFS=':' read -r key name script <<< "$agent_info"
        local script_path="$INTEGRATIONS_DIR/$script"

        if [[ -x "$script_path" ]]; then
            echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
            echo -e "${BLUE}Configuring:${NC} $name"
            echo ""
            if "$script_path" "$@" 2>/dev/null; then
                CONFIGURED+=("$name")
            else
                FAILED+=("$name")
            fi
        else
            log_warn "Script not found: $script_path"
            FAILED+=("$name")
        fi
    done
}

print_summary() {
    echo ""
    echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║${NC}                    Summary                                  ${CYAN}║${NC}"
    echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""

    echo "Detected: ${#DETECTED[@]} agent(s)"
    echo "Configured: ${#CONFIGURED[@]} agent(s)"

    if [[ ${#CONFIGURED[@]} -gt 0 ]]; then
        echo ""
        echo -e "${GREEN}Successfully configured:${NC}"
        for agent in "${CONFIGURED[@]}"; do
            echo "  • $agent"
        done
    fi

    if [[ ${#FAILED[@]} -gt 0 ]]; then
        echo ""
        echo -e "${RED}Failed to configure:${NC}"
        for agent in "${FAILED[@]}"; do
            echo "  • $agent"
        done
    fi

    echo ""
    echo "Server: $MCP_SERVER_PATH"
    echo "Port: $MCP_SERVER_PORT"
    echo ""
    echo "Next steps:"
    echo "  1. Restart your coding agents to load the new configuration"
    echo "  2. Mouchak Mail tools should now be available"
    echo ""
}

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Auto-detect installed coding agents and configure them to use Mouchak Mail.
Delegates to scripts in: scripts/integrations/

Options:
  -d, --detect-only     Only detect agents, don't configure
  -h, --help            Show this help message

Supported Agents:
  • Claude Code    (claude-code.sh)
  • Cursor IDE     (cursor.sh)
  • Windsurf IDE   (windsurf.sh)
  • Cline          (cline.sh)
  • GitHub Copilot (copilot.sh)
  • Continue       (continue.sh)
  • Aider          (aider.sh)
  • OpenCode       (opencode.sh)
  • Antigravity    (antigravity.sh)
  • Gemini CLI     (gemini.sh)
  • OpenAI Codex   (codex.sh)

Examples:
  $(basename "$0")                # Detect and configure all
  $(basename "$0") --detect-only  # Only show detected agents

Individual scripts:
  ./scripts/integrations/claude-code.sh --help
  ./scripts/integrations/cursor.sh --help
EOF
}

DETECT_ONLY=false
PASSTHROUGH_ARGS=()

while [[ $# -gt 0 ]]; do
    case $1 in
        -d|--detect-only)
            DETECT_ONLY=true
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            PASSTHROUGH_ARGS+=("$1")
            shift
            ;;
    esac
done

main() {
    print_header
    check_dependencies

    if ! find_mcp_server; then
        log_error "Cannot proceed without Mouchak Mail binary"
        exit 1
    fi

    if ! run_detection; then
        exit 0
    fi

    if [[ "$DETECT_ONLY" == true ]]; then
        echo ""
        log_info "Detection only mode - skipping configuration"
        exit 0
    fi

    ensure_server_running
    run_configuration "${PASSTHROUGH_ARGS[@]}"
    print_summary
}

main "$@"
