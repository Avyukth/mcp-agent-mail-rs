#!/usr/bin/env bash
# Unified Benchmark Script for Mouchak Mail
# Supports both Rust and Python implementations for head-to-head comparison
# Aligned with docs/UNIFIED_BENCHMARKING_PLAN.md
#
# Usage: ./scripts/benchmark_concurrent_agents.sh [OPTIONS]
#
# Options:
#   --impl rust|python   Select implementation (default: rust)
#   --port PORT          Server port (default: 8765)
#   --agents N           Concurrent connections (default: 100)
#   --duration N         Seconds per test (default: 10)
#   --mode MODE          Test mode: quick|standard|full|scaling|soak (default: standard)
#   --output DIR         Results directory (default: benchmark_results)
#
# Modes:
#   quick     - Health + MCP only (fast sanity check)
#   standard  - All 4 phases without scaling/soak
#   full      - All 6 categories including scaling
#   scaling   - Concurrency scaling tests only
#   soak      - Long-running stability test (1 hour)

set -euo pipefail

# ============================================
# Configuration
# ============================================
PORT="${PORT:-8765}"
AGENTS="${AGENTS:-100}"
DURATION="${DURATION:-10}"
IMPL="${IMPL:-rust}"
MODE="${MODE:-standard}"
HOST="http://127.0.0.1:${PORT}"
RESULTS_DIR="${RESULTS_DIR:-benchmark_results}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Endpoint mappings per implementation (bash 3.x compatible)
get_health_endpoint() {
    case "$1" in
        rust) echo "/health" ;;
        python) echo "/health/liveness" ;;
        *) echo "/health" ;;
    esac
}
get_ready_endpoint() {
    case "$1" in
        rust) echo "/ready" ;;
        python) echo "/health/readiness" ;;
        *) echo "/ready" ;;
    esac
}
MCP_ENDPOINT="/mcp"

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

# ============================================
# Parse Arguments
# ============================================
while [[ $# -gt 0 ]]; do
    case $1 in
        --port) PORT="$2"; HOST="http://127.0.0.1:${PORT}"; shift 2 ;;
        --agents) AGENTS="$2"; shift 2 ;;
        --duration) DURATION="$2"; shift 2 ;;
        --impl) IMPL="$2"; shift 2 ;;
        --mode) MODE="$2"; shift 2 ;;
        --output) RESULTS_DIR="$2"; shift 2 ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --impl rust|python   Select implementation (default: rust)"
            echo "  --port PORT          Server port (default: 8765)"
            echo "  --agents N           Concurrent connections (default: 100)"
            echo "  --duration N         Seconds per test (default: 10)"
            echo "  --mode MODE          Test mode: quick|standard|full|scaling|soak"
            echo "  --output DIR         Results directory (default: benchmark_results)"
            echo ""
            echo "Modes:"
            echo "  quick     - Health + MCP only (~1 min)"
            echo "  standard  - 4 phases: HTTP, DB, MCP, Messages (~5 min)"
            echo "  full      - All 6 categories including scaling (~15 min)"
            echo "  scaling   - Concurrency scaling tests only (~5 min)"
            echo "  soak      - 1-hour stability test"
            echo ""
            echo "Examples:"
            echo "  $0 --impl rust --mode quick"
            echo "  $0 --impl python --agents 50 --duration 30"
            echo "  $0 --mode full --output results/$(date +%Y%m%d)"
            exit 0
            ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

# Validate implementation
if [[ "$IMPL" != "rust" && "$IMPL" != "python" ]]; then
    echo "Error: Unknown implementation '$IMPL' (use rust or python)"
    exit 1
fi

HEALTH_ENDPOINT=$(get_health_endpoint "$IMPL")
READY_ENDPOINT=$(get_ready_endpoint "$IMPL")
IMPL_UPPER=$(echo "$IMPL" | tr '[:lower:]' '[:upper:]')

# Setup results directory
mkdir -p "$RESULTS_DIR"
RESULTS_FILE="${RESULTS_DIR}/benchmark_${IMPL}_${TIMESTAMP}.md"
JSON_FILE="${RESULTS_DIR}/benchmark_${IMPL}_${TIMESTAMP}.json"

# ============================================
# Utility Functions
# ============================================
log_header() {
    echo ""
    echo -e "${BOLD}${CYAN}=== $1 ===${NC}"
}

log_info() {
    echo -e "  ${CYAN}ℹ${NC} $1"
}

log_success() {
    echo -e "  ${GREEN}✓${NC} $1"
}

log_warning() {
    echo -e "  ${YELLOW}⚠${NC} $1"
}

log_error() {
    echo -e "  ${RED}✗${NC} $1"
}

# Check if hey is installed
check_dependencies() {
    if ! command -v hey &> /dev/null; then
        log_error "'hey' is not installed. Install with: brew install hey"
        exit 1
    fi
    if ! command -v curl &> /dev/null; then
        log_error "'curl' is not installed"
        exit 1
    fi
    if ! command -v bc &> /dev/null; then
        log_error "'bc' is not installed"
        exit 1
    fi
    log_success "Dependencies verified"
}

# Wait for server to be ready
wait_for_server() {
    echo "Waiting for server at ${HOST}${HEALTH_ENDPOINT}..."
    local max_attempts=30
    local attempt=0
    while ! curl -s "${HOST}${HEALTH_ENDPOINT}" > /dev/null 2>&1; do
        attempt=$((attempt + 1))
        if [ $attempt -ge $max_attempts ]; then
            log_error "Server did not become ready in time"
            exit 1
        fi
        sleep 1
        echo -n "."
    done
    echo ""
    log_success "Server is ready!"
}

# Parse hey output and extract metrics
parse_hey_output() {
    local output="$1"

    # Get status code counts
    local status_200=$(echo "$output" | grep -E '\[200\]' | awk '{print $2}')
    status_200=${status_200:-0}

    local status_errors=$(echo "$output" | grep -E '\[[45][0-9]{2}\]' | awk '{sum += $2} END {print sum+0}')
    status_errors=${status_errors:-0}

    local total=$((status_200 + status_errors))

    # Calculate success rate
    local success_rate
    if [ "$total" -gt 0 ]; then
        success_rate=$(echo "scale=1; $status_200 * 100 / $total" | bc)
    else
        success_rate="0"
    fi

    # Get latencies
    local p50=$(echo "$output" | grep "50% in" | awk '{print $3}')
    local p95=$(echo "$output" | grep "95% in" | awk '{print $3}')
    local p99=$(echo "$output" | grep "99% in" | awk '{print $3}')

    # Convert to ms
    local p50_ms="N/A"
    local p95_ms="N/A"
    local p99_ms="N/A"
    if [ -n "$p50" ]; then p50_ms=$(echo "$p50 * 1000 / 1" | bc); fi
    if [ -n "$p95" ]; then p95_ms=$(echo "$p95 * 1000 / 1" | bc); fi
    if [ -n "$p99" ]; then p99_ms=$(echo "$p99 * 1000 / 1" | bc); fi

    # Get request rate
    local rps=$(echo "$output" | grep "Requests/sec:" | awk '{print $2}' | cut -d'.' -f1)
    rps=${rps:-0}

    # Determine status
    local result_status result_color
    if [ "$success_rate" = "100.0" ] || [ "$success_rate" = "100" ]; then
        result_status="OK"
        result_color="$GREEN"
    elif (( $(echo "$success_rate >= 98" | bc -l) )); then
        result_status="EDGE"
        result_color="$YELLOW"
    else
        result_status="FAIL"
        result_color="$RED"
    fi

    # Return as associative array values
    echo "${rps}|${p50_ms}|${p95_ms}|${p99_ms}|${success_rate}|${total}|${result_status}"
}

# Run a benchmark test
run_benchmark() {
    local name="$1"
    local endpoint="$2"
    local method="${3:-GET}"
    local body="${4:-}"
    local rate="${5:-}"
    local custom_agents="${6:-$AGENTS}"
    local custom_duration="${7:-$DURATION}"

    echo ""
    echo "Testing: ${name}"
    if [ -n "$rate" ]; then
        echo "  Rate limit: ${rate} req/s"
    fi
    echo "  Concurrency: ${custom_agents} | Duration: ${custom_duration}s"
    echo "----------------------------------------"

    local args=("-c" "$custom_agents" "-z" "${custom_duration}s" "-t" "30")

    if [[ "$method" == "POST" ]]; then
        args+=("-m" "POST" "-H" "Content-Type: application/json")
        if [[ -n "$body" ]]; then
            args+=("-H" "Accept: application/json, text/event-stream")
            args+=("-d" "$body")
        fi
    fi

    if [ -n "$rate" ]; then
        args+=("-q" "$rate")
    fi

    args+=("${HOST}${endpoint}")

    # Run hey and capture output
    local output
    output=$(hey "${args[@]}" 2>&1)

    # Parse results
    local metrics
    metrics=$(parse_hey_output "$output")
    IFS='|' read -r rps p50 p95 p99 success total status <<< "$metrics"

    # Display results
    local status_color
    case "$status" in
        OK) status_color="$GREEN" ;;
        EDGE) status_color="$YELLOW" ;;
        *) status_color="$RED" ;;
    esac

    echo -e "  Rate: ${BOLD}${rps}${NC} req/s | P50: ${p50}ms | P99: ${p99}ms | Success: ${success}% | ${status_color}${status}${NC}"

    # Append to results file
    echo "| ${name} | ${rps} | ${p50}ms | ${p95}ms | ${p99}ms | ${success}% | ${status} |" >> "$RESULTS_FILE"
}

# Setup test data for message operations
setup_test_data() {
    log_info "Setting up test data..."

    # Create project
    curl -s -X POST "${HOST}/api/project/ensure" \
        -H "Content-Type: application/json" \
        -d '{"human_key": "benchmark-project"}' > /dev/null 2>&1 || true

    # Create agents
    curl -s -X POST "${HOST}/api/agent/register" \
        -H "Content-Type: application/json" \
        -d '{"project_slug":"benchmark-project","name":"sender","program":"bench","model":"test"}' > /dev/null 2>&1 || true

    curl -s -X POST "${HOST}/api/agent/register" \
        -H "Content-Type: application/json" \
        -d '{"project_slug":"benchmark-project","name":"receiver","program":"bench","model":"test"}' > /dev/null 2>&1 || true

    log_success "Test data ready (project: benchmark-project, agents: sender/receiver)"
}

# ============================================
# Benchmark Phases
# ============================================

# Phase 1: Raw HTTP Throughput
phase_http() {
    log_header "Phase 1: Raw HTTP Throughput"
    run_benchmark "Health (no I/O)" "$HEALTH_ENDPOINT"
    run_benchmark "Health @ 5000/s" "$HEALTH_ENDPOINT" "GET" "" "5000"
    run_benchmark "Health @ 2000/s" "$HEALTH_ENDPOINT" "GET" "" "2000"
}

# Phase 2: Database Operations
phase_database() {
    log_header "Phase 2: Database Operations"
    run_benchmark "Ready (DB check)" "$READY_ENDPOINT"
    run_benchmark "Ready @ 2000/s" "$READY_ENDPOINT" "GET" "" "2000"
    run_benchmark "Ready @ 1000/s" "$READY_ENDPOINT" "GET" "" "1000"
}

# Phase 3: MCP Protocol
phase_mcp() {
    log_header "Phase 3: MCP Protocol"
    local mcp_body='{"jsonrpc":"2.0","method":"tools/list","params":{},"id":1}'
    run_benchmark "MCP tools/list" "$MCP_ENDPOINT" "POST" "$mcp_body"
    run_benchmark "MCP @ 2000/s" "$MCP_ENDPOINT" "POST" "$mcp_body" "2000"
    run_benchmark "MCP @ 1000/s" "$MCP_ENDPOINT" "POST" "$mcp_body" "1000"
    run_benchmark "MCP @ 500/s" "$MCP_ENDPOINT" "POST" "$mcp_body" "500"
}

# Phase 4: Message Operations
phase_messages() {
    log_header "Phase 4: Message Operations"
    setup_test_data

    local msg_body='{"project_slug":"benchmark-project","sender_name":"sender","recipient_names":["receiver"],"subject":"Bench","body_md":"Test message content"}'
    local inbox_body='{"project_slug":"benchmark-project","agent_name":"receiver"}'

    run_benchmark "Message Send" "/api/message/send" "POST" "$msg_body"
    run_benchmark "Message Send @ 500/s" "/api/message/send" "POST" "$msg_body" "500"
    run_benchmark "Inbox Fetch" "/api/inbox" "POST" "$inbox_body"
    run_benchmark "Inbox Fetch @ 1000/s" "/api/inbox" "POST" "$inbox_body" "1000"
}

# Phase 5: Concurrency Scaling
phase_scaling() {
    log_header "Phase 5: Concurrency Scaling"
    log_info "Testing throughput at different concurrency levels"

    for agents in 10 25 50 100 200 500; do
        run_benchmark "Health @ ${agents}c" "$HEALTH_ENDPOINT" "GET" "" "" "$agents" "5"
    done
}

# Phase 6: Soak Testing
phase_soak() {
    log_header "Phase 6: Soak Testing (1 hour)"
    log_warning "This will take approximately 1 hour"

    run_benchmark "Soak Test (1hr)" "$HEALTH_ENDPOINT" "GET" "" "" "50" "3600"
}

# ============================================
# Initialize Results File
# ============================================
init_results_file() {
    cat > "$RESULTS_FILE" << EOF
# Benchmark Results: ${IMPL_UPPER} Implementation

**Date:** $(date)
**Target:** ${HOST}
**Duration:** ${DURATION}s per test
**Concurrency:** ${AGENTS} agents
**Implementation:** ${IMPL}
**Mode:** ${MODE}

## Results

| Test | Throughput (req/s) | P50 | P95 | P99 | Success | Status |
|------|-------------------|-----|-----|-----|---------|--------|
EOF
}

# Finalize results file
finalize_results_file() {
    cat >> "$RESULTS_FILE" << EOF

## Environment

- **Implementation:** ${IMPL_UPPER}
- **Health Endpoint:** ${HEALTH_ENDPOINT}
- **Ready Endpoint:** ${READY_ENDPOINT}
- **MCP Endpoint:** ${MCP_ENDPOINT}
- **Server:** ${HOST}

## Comparison Notes

To compare implementations, run:
\`\`\`bash
# Rust
./scripts/benchmark_concurrent_agents.sh --impl rust --mode standard

# Python
./scripts/benchmark_concurrent_agents.sh --impl python --mode standard
\`\`\`

Then compare the results in \`${RESULTS_DIR}/\`

---
*Generated by benchmark_concurrent_agents.sh (aligned with UNIFIED_BENCHMARKING_PLAN.md)*
EOF
}

# ============================================
# Main Execution
# ============================================

echo "=============================================="
echo -e "${BOLD}Mouchak Mail Benchmark - ${CYAN}${IMPL_UPPER}${NC}${BOLD} Implementation${NC}"
echo "=============================================="
echo "Target: ${HOST}"
echo "Mode: ${MODE}"
echo "Duration: ${DURATION}s per test"
echo "Concurrency: ${AGENTS} agents"
echo "Results: ${RESULTS_FILE}"
echo ""

check_dependencies
wait_for_server
init_results_file

case "$MODE" in
    quick)
        phase_http
        phase_mcp
        ;;
    standard)
        phase_http
        phase_database
        phase_mcp
        phase_messages
        ;;
    full)
        phase_http
        phase_database
        phase_mcp
        phase_messages
        phase_scaling
        ;;
    scaling)
        phase_scaling
        ;;
    soak)
        phase_soak
        ;;
    *)
        log_error "Unknown mode: $MODE"
        exit 1
        ;;
esac

finalize_results_file

echo ""
echo "=============================================="
echo -e "${GREEN}${BOLD}Benchmark Complete!${NC}"
echo "=============================================="
echo "Results saved to: ${RESULTS_FILE}"
echo ""

# Display summary
echo -e "${BOLD}Quick Summary:${NC}"
tail -n +12 "$RESULTS_FILE" | head -20
