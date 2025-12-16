#!/usr/bin/env bash
# Agent Workflow Benchmark Script
# Tests realistic multi-step agent operations for MCP Agent Mail
#
# This script benchmarks ACTUAL AGENT WORKFLOWS, not just raw throughput.
# It measures what matters for AI agents: end-to-end operation latency.
#
# Key Insight: Rust uses REST API (/api/*), Python uses MCP protocol (/mcp)
# Both are valid implementations - this benchmark tests equivalent operations.
#
# Usage: ./scripts/benchmark_agent_workflows.sh [OPTIONS]
#
# Options:
#   --impl rust|python   Select implementation (default: rust)
#   --port PORT          Server port (default: 8765)
#   --iterations N       Iterations per test (default: 100)
#   --output DIR         Results directory (default: benchmark_results)

set -euo pipefail

# ============================================
# Configuration
# ============================================
PORT="${PORT:-8765}"
IMPL="${IMPL:-rust}"
ITERATIONS="${ITERATIONS:-100}"
HOST="http://127.0.0.1:${PORT}"
RESULTS_DIR="${RESULTS_DIR:-benchmark_results}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

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
        --impl) IMPL="$2"; shift 2 ;;
        --iterations) ITERATIONS="$2"; shift 2 ;;
        --output) RESULTS_DIR="$2"; shift 2 ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --impl rust|python   Select implementation (default: rust)"
            echo "  --port PORT          Server port (default: 8765)"
            echo "  --iterations N       Iterations per test (default: 100)"
            echo "  --output DIR         Results directory"
            echo ""
            echo "Examples:"
            echo "  $0 --impl rust --iterations 50"
            echo "  $0 --impl python --iterations 100"
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

IMPL_UPPER=$(echo "$IMPL" | tr '[:lower:]' '[:upper:]')
mkdir -p "$RESULTS_DIR"
RESULTS_FILE="${RESULTS_DIR}/workflow_${IMPL}_${TIMESTAMP}.md"

# ============================================
# Implementation-specific endpoints
# ============================================
if [[ "$IMPL" == "rust" ]]; then
    HEALTH_ENDPOINT="/health"
    PROJECT_ENDPOINT="/api/project/ensure"
    AGENT_ENDPOINT="/api/agent/register"
    SEND_ENDPOINT="/api/message/send"
    INBOX_ENDPOINT="/api/inbox"
else
    HEALTH_ENDPOINT="/health/liveness"
    MCP_ENDPOINT="/mcp"
fi

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

log_error() {
    echo -e "  ${RED}✗${NC} $1"
}

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

# Measure single request latency in milliseconds
measure_latency() {
    local start=$(python3 -c "import time; print(int(time.time() * 1000))")
    eval "$1" > /dev/null 2>&1
    local end=$(python3 -c "import time; print(int(time.time() * 1000))")
    echo $((end - start))
}

# Calculate statistics from an array of latencies
calc_stats() {
    local -n arr=$1
    local count=${#arr[@]}

    if [ $count -eq 0 ]; then
        echo "N/A|N/A|N/A|N/A"
        return
    fi

    # Sort array
    IFS=$'\n' sorted=($(sort -n <<< "${arr[*]}")); unset IFS

    # Calculate mean
    local sum=0
    for val in "${arr[@]}"; do
        sum=$((sum + val))
    done
    local mean=$((sum / count))

    # Get percentiles
    local p50_idx=$(( (count * 50) / 100 ))
    local p95_idx=$(( (count * 95) / 100 ))
    local p99_idx=$(( (count * 99) / 100 ))

    local p50=${sorted[$p50_idx]}
    local p95=${sorted[$p95_idx]}
    local p99=${sorted[$p99_idx]}

    echo "${mean}|${p50}|${p95}|${p99}"
}

# ============================================
# Rust REST API Tests
# ============================================
rust_ensure_project() {
    curl -s -X POST "${HOST}/api/project/ensure" \
        -H "Content-Type: application/json" \
        -d '{"human_key": "benchmark-workflow"}'
}

rust_register_agent() {
    local name="$1"
    curl -s -X POST "${HOST}/api/agent/register" \
        -H "Content-Type: application/json" \
        -d "{\"project_slug\":\"benchmark-workflow\",\"name\":\"${name}\",\"program\":\"bench\",\"model\":\"test\"}"
}

rust_send_message() {
    curl -s -X POST "${HOST}/api/message/send" \
        -H "Content-Type: application/json" \
        -d '{"project_slug":"benchmark-workflow","sender_name":"agent-a","recipient_names":["agent-b"],"subject":"Test","body_md":"Hello"}'
}

rust_check_inbox() {
    curl -s -X POST "${HOST}/api/inbox" \
        -H "Content-Type: application/json" \
        -d '{"project_slug":"benchmark-workflow","agent_name":"agent-b"}'
}

# ============================================
# Python MCP Protocol Tests
# ============================================
python_mcp_call() {
    local tool_name="$1"
    local arguments="$2"
    curl -s -X POST "${HOST}/mcp" \
        -H "Content-Type: application/json" \
        -H "Accept: application/json, text/event-stream" \
        -d "{\"jsonrpc\":\"2.0\",\"method\":\"tools/call\",\"params\":{\"name\":\"${tool_name}\",\"arguments\":${arguments}},\"id\":1}"
}

python_ensure_project() {
    python_mcp_call "ensure_project" '{"human_key":"benchmark-workflow"}'
}

python_register_agent() {
    local name="$1"
    python_mcp_call "register_agent" "{\"project_slug\":\"benchmark-workflow\",\"name\":\"${name}\",\"program\":\"bench\",\"model\":\"test\"}"
}

python_send_message() {
    python_mcp_call "send_message" '{"project_slug":"benchmark-workflow","sender_name":"agent-a","recipient_names":["agent-b"],"subject":"Test","body_md":"Hello"}'
}

python_check_inbox() {
    python_mcp_call "fetch_inbox" '{"project_slug":"benchmark-workflow","agent_name":"agent-b"}'
}

# ============================================
# Workflow Benchmarks
# ============================================
benchmark_workflow() {
    local name="$1"
    local description="$2"
    shift 2
    local commands=("$@")

    echo ""
    echo "Testing: ${name}"
    echo "  Description: ${description}"
    echo "  Iterations: ${ITERATIONS}"
    echo "----------------------------------------"

    local latencies=()
    local successes=0
    local failures=0

    for ((i=1; i<=ITERATIONS; i++)); do
        local total_latency=0
        local workflow_success=true

        for cmd in "${commands[@]}"; do
            local start=$(python3 -c "import time; print(int(time.time() * 1000))")
            if eval "$cmd" > /dev/null 2>&1; then
                local end=$(python3 -c "import time; print(int(time.time() * 1000))")
                total_latency=$((total_latency + end - start))
            else
                workflow_success=false
                break
            fi
        done

        if [ "$workflow_success" = true ]; then
            latencies+=($total_latency)
            ((successes++))
        else
            ((failures++))
        fi

        # Progress indicator every 10 iterations
        if ((i % 10 == 0)); then
            echo -n "."
        fi
    done
    echo ""

    # Calculate statistics
    local stats
    stats=$(calc_stats latencies)
    IFS='|' read -r mean p50 p95 p99 <<< "$stats"

    local success_rate
    if [ $ITERATIONS -gt 0 ]; then
        success_rate=$(echo "scale=1; $successes * 100 / $ITERATIONS" | bc)
    else
        success_rate="0"
    fi

    # Determine status
    local status status_color
    if [ "$success_rate" = "100.0" ] || [ "$success_rate" = "100" ]; then
        status="OK"
        status_color="$GREEN"
    elif (( $(echo "$success_rate >= 98" | bc -l) )); then
        status="EDGE"
        status_color="$YELLOW"
    else
        status="FAIL"
        status_color="$RED"
    fi

    echo -e "  Mean: ${BOLD}${mean}ms${NC} | P50: ${p50}ms | P95: ${p95}ms | P99: ${p99}ms | ${status_color}${status}${NC}"

    # Append to results file
    echo "| ${name} | ${mean}ms | ${p50}ms | ${p95}ms | ${p99}ms | ${success_rate}% | ${status} |" >> "$RESULTS_FILE"
}

# ============================================
# Setup Test Environment
# ============================================
setup_test_env() {
    log_info "Setting up test environment..."

    if [[ "$IMPL" == "rust" ]]; then
        rust_ensure_project > /dev/null 2>&1 || true
        rust_register_agent "agent-a" > /dev/null 2>&1 || true
        rust_register_agent "agent-b" > /dev/null 2>&1 || true
    else
        python_ensure_project > /dev/null 2>&1 || true
        python_register_agent "agent-a" > /dev/null 2>&1 || true
        python_register_agent "agent-b" > /dev/null 2>&1 || true
    fi

    log_success "Test environment ready"
}

# ============================================
# Initialize Results File
# ============================================
init_results_file() {
    cat > "$RESULTS_FILE" << EOF
# Agent Workflow Benchmark: ${IMPL_UPPER} Implementation

**Date:** $(date)
**Target:** ${HOST}
**Iterations:** ${ITERATIONS} per workflow
**Implementation:** ${IMPL}

## Architecture Note

EOF

    if [[ "$IMPL" == "rust" ]]; then
        cat >> "$RESULTS_FILE" << EOF
This benchmark tests the **Rust REST API** implementation.
- Endpoints: \`/api/project/ensure\`, \`/api/agent/register\`, \`/api/message/send\`, \`/api/inbox\`
- Protocol: Direct HTTP REST calls (JSON request/response)
- No MCP JSON-RPC overhead

EOF
    else
        cat >> "$RESULTS_FILE" << EOF
This benchmark tests the **Python MCP Protocol** implementation.
- Endpoint: \`/mcp\` (all operations)
- Protocol: JSON-RPC 2.0 via MCP tools/call
- Each request creates a new StreamableHTTPServerTransport instance

EOF
    fi

    cat >> "$RESULTS_FILE" << EOF
## Results

| Workflow | Mean | P50 | P95 | P99 | Success | Status |
|----------|------|-----|-----|-----|---------|--------|
EOF
}

# ============================================
# Main Execution
# ============================================
echo "=============================================="
echo -e "${BOLD}Agent Workflow Benchmark - ${CYAN}${IMPL_UPPER}${NC}${BOLD} Implementation${NC}"
echo "=============================================="
echo "Target: ${HOST}"
echo "Iterations: ${ITERATIONS} per workflow"
echo "Results: ${RESULTS_FILE}"
echo ""

wait_for_server
init_results_file
setup_test_env

# ============================================
# Run Workflow Benchmarks
# ============================================
log_header "Workflow 1: Single Operations"

if [[ "$IMPL" == "rust" ]]; then
    benchmark_workflow "Project Ensure" "Create or get project" \
        "rust_ensure_project"

    benchmark_workflow "Agent Register" "Register new agent" \
        "rust_register_agent agent-test-\$i"

    benchmark_workflow "Send Message" "Send single message" \
        "rust_send_message"

    benchmark_workflow "Check Inbox" "Fetch agent inbox" \
        "rust_check_inbox"
else
    benchmark_workflow "Project Ensure" "Create or get project (MCP)" \
        "python_ensure_project"

    benchmark_workflow "Agent Register" "Register new agent (MCP)" \
        "python_register_agent agent-test-\$i"

    benchmark_workflow "Send Message" "Send single message (MCP)" \
        "python_send_message"

    benchmark_workflow "Check Inbox" "Fetch agent inbox (MCP)" \
        "python_check_inbox"
fi

log_header "Workflow 2: Agent Communication Cycle"

if [[ "$IMPL" == "rust" ]]; then
    benchmark_workflow "Send+Receive" "Agent A sends, Agent B checks inbox" \
        "rust_send_message" "rust_check_inbox"

    benchmark_workflow "Full Conversation" "Send → Check → Reply → Check" \
        "rust_send_message" "rust_check_inbox" \
        "curl -s -X POST '${HOST}/api/message/send' -H 'Content-Type: application/json' -d '{\"project_slug\":\"benchmark-workflow\",\"sender_name\":\"agent-b\",\"recipient_names\":[\"agent-a\"],\"subject\":\"Re: Test\",\"body_md\":\"Reply\"}'" \
        "curl -s -X POST '${HOST}/api/inbox' -H 'Content-Type: application/json' -d '{\"project_slug\":\"benchmark-workflow\",\"agent_name\":\"agent-a\"}'"
else
    benchmark_workflow "Send+Receive" "Agent A sends, Agent B checks inbox (MCP)" \
        "python_send_message" "python_check_inbox"

    benchmark_workflow "Full Conversation" "Send → Check → Reply → Check (MCP)" \
        "python_send_message" "python_check_inbox" \
        "python_mcp_call send_message '{\"project_slug\":\"benchmark-workflow\",\"sender_name\":\"agent-b\",\"recipient_names\":[\"agent-a\"],\"subject\":\"Re: Test\",\"body_md\":\"Reply\"}'" \
        "python_mcp_call fetch_inbox '{\"project_slug\":\"benchmark-workflow\",\"agent_name\":\"agent-a\"}'"
fi

log_header "Workflow 3: Agent Onboarding"

if [[ "$IMPL" == "rust" ]]; then
    benchmark_workflow "New Agent Setup" "Project + Register + First Message" \
        "rust_ensure_project" \
        "rust_register_agent onboard-\$RANDOM" \
        "rust_send_message"
else
    benchmark_workflow "New Agent Setup" "Project + Register + First Message (MCP)" \
        "python_ensure_project" \
        "python_register_agent onboard-\$RANDOM" \
        "python_send_message"
fi

# ============================================
# Finalize Results
# ============================================
cat >> "$RESULTS_FILE" << EOF

## Interpretation

These benchmarks measure **end-to-end workflow latency** - what an AI agent actually experiences.

### Key Metrics

- **Mean**: Average latency across all iterations
- **P50**: Median latency (50th percentile)
- **P95**: 95th percentile (worst 5% excluded)
- **P99**: 99th percentile (worst 1% excluded)

### What This Means for AI Agents

1. **LLM Bottleneck**: AI inference takes 10-60 seconds per turn
2. **Backend Latency**: These operations add to that baseline
3. **Practical Impact**:
   - <50ms: Imperceptible to users
   - 50-200ms: Acceptable overhead
   - >500ms: May need optimization for interactive use

### Comparison Notes

To compare implementations:
\`\`\`bash
# Rust (REST API)
./scripts/benchmark_agent_workflows.sh --impl rust --iterations 100

# Python (MCP Protocol)
./scripts/benchmark_agent_workflows.sh --impl python --iterations 100
\`\`\`

---
*Generated by benchmark_agent_workflows.sh*
EOF

echo ""
echo "=============================================="
echo -e "${GREEN}${BOLD}Benchmark Complete!${NC}"
echo "=============================================="
echo "Results saved to: ${RESULTS_FILE}"
echo ""
echo -e "${BOLD}Quick Summary:${NC}"
tail -n +20 "$RESULTS_FILE" | head -15
