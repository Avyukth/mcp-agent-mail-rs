# Unified Benchmarking Plan: Rust vs Python MCP Agent Mail

> **Generated:** 2025-12-16
> **Scope:** Head-to-head comparison of Rust (Axum) and Python (FastAPI) implementations
> **Goal:** Apples-to-apples benchmarking with identical parameters

---

## Executive Summary

This document provides a unified benchmarking framework for comparing the **Rust** (Axum/libsql/Git2) and **Python** (FastAPI/SQLAlchemy/GitPython) implementations of MCP Agent Mail. All tests use identical parameters, endpoints, and metrics to enable direct comparison.

### Quick Results Reference (100 Concurrent Agents, 5s Duration)

| Endpoint | Rust | Python | Rust Advantage |
|----------|------|--------|----------------|
| Health | 61,809 req/s (P99: 10ms) | 7,624 req/s (P99: 38ms) | **8.1x** throughput |
| DB Ready | 58,051 req/s (P99: 10ms) | 2,063 req/s (P99: 159ms) | **28x** throughput |
| MCP tools/list | 59,144 req/s (P99: 11ms) | 314 req/s (P99: 836ms) | **188x** throughput |

---

## Table of Contents

1. [Architecture Comparison](#1-architecture-comparison)
2. [Endpoint Mapping](#2-endpoint-mapping)
3. [Test Parameters](#3-test-parameters)
4. [Benchmark Categories](#4-benchmark-categories)
5. [Test Execution](#5-test-execution)
6. [Metrics Collection](#6-metrics-collection)
7. [Analysis Framework](#7-analysis-framework)
8. [Automation Scripts](#8-automation-scripts)

---

## 1. Architecture Comparison

### 1.1 System Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                            LOAD GENERATOR                                    │
│                        (hey / k6 / wrk / criterion)                         │
└────────────────────────────────────┬────────────────────────────────────────┘
                                     │
                 ┌───────────────────┴───────────────────┐
                 ▼                                       ▼
┌────────────────────────────────────┐   ┌────────────────────────────────────┐
│        RUST IMPLEMENTATION         │   │       PYTHON IMPLEMENTATION        │
│                                    │   │                                    │
│  ┌──────────────────────────────┐  │   │  ┌──────────────────────────────┐  │
│  │      Rate Limiter            │  │   │  │      Rate Limiter            │  │
│  │   governor (configurable)    │  │   │  │   slowapi (configurable)     │  │
│  └──────────────────────────────┘  │   │  └──────────────────────────────┘  │
│               │                    │   │               │                    │
│  ┌──────────────────────────────┐  │   │  ┌──────────────────────────────┐  │
│  │      HTTP Server             │  │   │  │      HTTP Server             │  │
│  │   Axum 0.8 + Tokio           │  │   │  │   FastAPI + Uvicorn          │  │
│  └──────────────────────────────┘  │   │  └──────────────────────────────┘  │
│               │                    │   │               │                    │
│  ┌──────────────────────────────┐  │   │  ┌──────────────────────────────┐  │
│  │      MCP Transport           │  │   │  │      MCP Transport           │  │
│  │   rmcp (Rust SDK)            │  │   │  │   FastMCP + SSE              │  │
│  └──────────────────────────────┘  │   │  └──────────────────────────────┘  │
│               │                    │   │               │                    │
│  ┌──────────────────────────────┐  │   │  ┌──────────────────────────────┐  │
│  │      Business Logic          │  │   │  │      Business Logic          │  │
│  │   lib-core (BMC pattern)     │  │   │  │   app.py (MCP tools)         │  │
│  └──────────────────────────────┘  │   │  └──────────────────────────────┘  │
│               │                    │   │               │                    │
│  ┌─────────────┬────────────────┐  │   │  ┌─────────────┬────────────────┐  │
│  │   libsql    │     Git2       │  │   │  │ SQLAlchemy  │   GitPython    │  │
│  │ (SQLite+WAL)│  (archive)     │  │   │  │  (SQLite)   │   (archive)    │  │
│  └─────────────┴────────────────┘  │   │  └─────────────┴────────────────┘  │
│                                    │   │                                    │
│  Port: 8765                        │   │  Port: 8765                        │
│  Data: ./data/                     │   │  Data: ./data/                     │
└────────────────────────────────────┘   └────────────────────────────────────┘
```

### 1.2 Technology Stack Comparison

| Layer | Rust | Python |
|-------|------|--------|
| **Runtime** | Tokio (async) | asyncio + uvloop |
| **HTTP Framework** | Axum 0.8 | FastAPI 0.115 |
| **ASGI/Server** | hyper (built-in) | Uvicorn |
| **Database** | libsql (SQLite fork) | SQLAlchemy + aiosqlite |
| **Git** | git2 (libgit2 bindings) | GitPython |
| **MCP** | rmcp | FastMCP |
| **Rate Limiting** | governor | slowapi |
| **JSON** | serde_json | orjson/ujson |
| **Workers** | Single-threaded Tokio | Multi-worker Gunicorn |

---

## 2. Endpoint Mapping

### 2.1 Health/Readiness Endpoints

| Function | Rust Endpoint | Python Endpoint |
|----------|---------------|-----------------|
| Liveness (no I/O) | `GET /health` | `GET /health/liveness` |
| Readiness (DB check) | `GET /ready` | `GET /health/readiness` |

### 2.2 MCP Endpoints

| Function | Rust Endpoint | Python Endpoint |
|----------|---------------|-----------------|
| MCP JSON-RPC | `POST /mcp` | `POST /mcp` |
| MCP SSE Stream | `POST /mcp` (Accept: text/event-stream) | `POST /mcp` (Accept: text/event-stream) |

### 2.3 REST API Endpoints

| Function | Rust Endpoint | Python Endpoint |
|----------|---------------|-----------------|
| Fetch Inbox | `POST /api/inbox` | `POST /api/inbox` or MCP tool |
| Send Message | `POST /api/message/send` | `POST /api/message/send` or MCP tool |
| Register Agent | `POST /api/agent/register` | `POST /api/agent/register` |
| Ensure Project | `POST /api/project/ensure` | `POST /api/project/ensure` |
| Search Messages | `POST /api/messages/search` | `POST /api/messages/search` |

---

## 3. Test Parameters

### 3.1 Standard Test Configuration

All benchmarks MUST use these identical parameters for fair comparison:

```yaml
# Standard Parameters
concurrency:
  agents: [10, 25, 50, 100, 200, 500]
  default: 100

duration:
  quick: 5s
  standard: 30s
  extended: 5m
  soak: 1h

rate_limiting:
  disabled: true  # For max throughput tests
  enabled: false  # Default for comparison

data_setup:
  projects: 10
  agents_per_project: 100
  messages_per_agent: 100

environment:
  port: 8765
  host: "127.0.0.1"
  data_dir: "./data"
```

### 3.2 Server Startup Commands

```bash
# ============================================
# RUST SERVER
# ============================================
cd /path/to/mcp-agent-mail-rs
cargo build --release -p mcp-server

# Clean start (fresh database)
rm -rf data && mkdir -p data/archive
RATE_LIMIT_ENABLED=false ./target/release/mcp-server

# ============================================
# PYTHON SERVER
# ============================================
cd /path/to/temp_mcp_mail

# Clean start (fresh database)
rm -rf data && mkdir -p data

# Single worker (fair comparison with Rust single-threaded)
uv run python -m mcp_agent_mail.http --host 0.0.0.0 --port 8765

# Multi-worker (production config)
uv run gunicorn -w 4 -k uvicorn.workers.UvicornWorker \
  mcp_agent_mail.http:build_http_app --bind 0.0.0.0:8765
```

### 3.3 Environment Variables

| Variable | Rust | Python | Description |
|----------|------|--------|-------------|
| `PORT` | `PORT=8765` | `--port 8765` | Server port |
| `RATE_LIMIT_ENABLED` | `RATE_LIMIT_ENABLED=false` | N/A (code change) | Disable rate limiting |
| `DATABASE_ECHO` | `RUST_LOG=debug` | `DATABASE_ECHO=true` | SQL logging |
| `LOG_LEVEL` | `RUST_LOG=info` | `LOG_LEVEL=INFO` | General logging |

---

## 4. Benchmark Categories

### 4.1 Category 1: Raw HTTP Throughput

**Purpose:** Measure HTTP framework overhead without business logic.

```bash
# Test health endpoints (no I/O)
RUST:   hey -c 100 -z 30s http://127.0.0.1:8765/health
PYTHON: hey -c 100 -z 30s http://127.0.0.1:8765/health/liveness
```

**Metrics:**
- Requests/second
- P50, P95, P99, P999 latency
- Error rate

### 4.2 Category 2: Database Operations

**Purpose:** Measure database connection pool and query performance.

```bash
# Test readiness endpoints (DB query)
RUST:   hey -c 100 -z 30s http://127.0.0.1:8765/ready
PYTHON: hey -c 100 -z 30s http://127.0.0.1:8765/health/readiness
```

**Metrics:**
- Requests/second
- P99 latency
- Connection pool saturation

### 4.3 Category 3: MCP Protocol

**Purpose:** Measure MCP JSON-RPC processing overhead.

```bash
# Test MCP tools/list (read-only, no DB)
hey -c 100 -z 30s -m POST \
  -H "Content-Type: application/json" \
  -H "Accept: application/json, text/event-stream" \
  -d '{"jsonrpc":"2.0","method":"tools/list","params":{},"id":1}' \
  http://127.0.0.1:8765/mcp
```

**Metrics:**
- Requests/second
- P99 latency
- Transport instantiation overhead

### 4.4 Category 4: Message Operations

**Purpose:** Measure real-world message send/receive performance.

```bash
# Pre-setup: Create project and agents
curl -X POST http://127.0.0.1:8765/api/project/ensure \
  -H "Content-Type: application/json" \
  -d '{"human_key": "benchmark-project"}'

curl -X POST http://127.0.0.1:8765/api/agent/register \
  -H "Content-Type: application/json" \
  -d '{"project_slug":"benchmark-project","name":"sender","program":"test","model":"test"}'

curl -X POST http://127.0.0.1:8765/api/agent/register \
  -H "Content-Type: application/json" \
  -d '{"project_slug":"benchmark-project","name":"receiver","program":"test","model":"test"}'

# Benchmark: Send messages
hey -c 100 -z 30s -m POST \
  -H "Content-Type: application/json" \
  -d '{"project_slug":"benchmark-project","sender_name":"sender","recipient_names":["receiver"],"subject":"Test","body_md":"Content"}' \
  http://127.0.0.1:8765/api/message/send

# Benchmark: Fetch inbox
hey -c 100 -z 30s -m POST \
  -H "Content-Type: application/json" \
  -d '{"project_slug":"benchmark-project","agent_name":"receiver"}' \
  http://127.0.0.1:8765/api/inbox
```

**Metrics:**
- Messages/second (write)
- Inbox fetches/second (read)
- P99 latency
- Git commit overhead

### 4.5 Category 5: Concurrency Scaling

**Purpose:** Measure performance degradation under increasing load.

```bash
# Test with increasing concurrent agents
for agents in 10 25 50 100 200 500; do
  echo "=== Testing with $agents agents ==="
  hey -c $agents -z 10s http://127.0.0.1:8765/health
done
```

**Metrics:**
- Throughput at each concurrency level
- Latency increase curve
- Breaking point identification

### 4.6 Category 6: Soak Testing

**Purpose:** Identify memory leaks and long-running stability issues.

```bash
# 1-hour sustained load
hey -c 50 -z 1h http://127.0.0.1:8765/health

# Monitor during test
watch -n 5 'ps -o rss,vsz,pid,command -p $(pgrep -f "mcp-server\|uvicorn")'
```

**Metrics:**
- Memory growth over time
- P99 latency drift
- Error rate over time
- File descriptor count

---

## 5. Test Execution

### 5.1 Unified Benchmark Script

Save as `scripts/unified_benchmark.sh`:

```bash
#!/usr/bin/env bash
# Unified Benchmark Script for Rust vs Python MCP Agent Mail
# Usage: ./scripts/unified_benchmark.sh --impl rust|python [OPTIONS]

set -euo pipefail

# ============================================
# Configuration
# ============================================
PORT="${PORT:-8765}"
AGENTS="${AGENTS:-100}"
DURATION="${DURATION:-30}"
IMPL="${IMPL:-rust}"
HOST="http://127.0.0.1:${PORT}"
RESULTS_DIR="benchmark_results"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULTS_FILE="${RESULTS_DIR}/benchmark_${IMPL}_${TIMESTAMP}.md"

# Endpoint mappings
declare -A HEALTH_ENDPOINTS=(
    ["rust"]="/health"
    ["python"]="/health/liveness"
)
declare -A READY_ENDPOINTS=(
    ["rust"]="/ready"
    ["python"]="/health/readiness"
)

# ============================================
# Parse Arguments
# ============================================
while [[ $# -gt 0 ]]; do
    case $1 in
        --impl) IMPL="$2"; shift 2 ;;
        --port) PORT="$2"; HOST="http://127.0.0.1:${PORT}"; shift 2 ;;
        --agents) AGENTS="$2"; shift 2 ;;
        --duration) DURATION="$2"; shift 2 ;;
        --output) RESULTS_FILE="$2"; shift 2 ;;
        -h|--help)
            echo "Usage: $0 --impl rust|python [--port PORT] [--agents N] [--duration N]"
            exit 0
            ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

HEALTH_ENDPOINT="${HEALTH_ENDPOINTS[$IMPL]}"
READY_ENDPOINT="${READY_ENDPOINTS[$IMPL]}"

# ============================================
# Setup
# ============================================
mkdir -p "$RESULTS_DIR"

echo "=============================================="
echo "Unified MCP Agent Mail Benchmark"
echo "=============================================="
echo "Implementation: ${IMPL^^}"
echo "Target: ${HOST}"
echo "Concurrency: ${AGENTS} agents"
echo "Duration: ${DURATION}s per test"
echo ""

# Check dependencies
command -v hey >/dev/null 2>&1 || { echo "Error: 'hey' not installed"; exit 1; }
command -v jq >/dev/null 2>&1 || { echo "Warning: 'jq' not installed"; }

# Wait for server
wait_for_server() {
    echo "Waiting for server..."
    for i in {1..30}; do
        if curl -s "${HOST}${HEALTH_ENDPOINT}" >/dev/null 2>&1; then
            echo "Server ready!"
            return 0
        fi
        sleep 1
    done
    echo "Error: Server not ready"
    exit 1
}

# ============================================
# Benchmark Functions
# ============================================
run_benchmark() {
    local name="$1"
    local endpoint="$2"
    local method="${3:-GET}"
    local body="${4:-}"
    local content_type="${5:-application/json}"

    echo ""
    echo "=== ${name} ==="

    local args=("-c" "$AGENTS" "-z" "${DURATION}s" "-t" "30")

    if [[ "$method" == "POST" ]]; then
        args+=("-m" "POST" "-H" "Content-Type: ${content_type}")
        if [[ -n "$body" ]]; then
            args+=("-d" "$body")
        fi
    fi

    args+=("${HOST}${endpoint}")

    local output
    output=$(hey "${args[@]}" 2>&1)

    # Parse results
    local rps p99 status_200 status_errors success_rate
    rps=$(echo "$output" | grep "Requests/sec:" | awk '{print $2}' | cut -d'.' -f1)
    p99=$(echo "$output" | grep "99% in" | awk '{print $3}')
    status_200=$(echo "$output" | grep -E '\[200\]' | awk '{print $2}')
    status_errors=$(echo "$output" | grep -E '\[[45][0-9]{2}\]' | awk '{sum += $2} END {print sum+0}')

    local total=$((${status_200:-0} + ${status_errors:-0}))
    if [[ $total -gt 0 ]]; then
        success_rate=$(echo "scale=1; ${status_200:-0} * 100 / $total" | bc)
    else
        success_rate="0"
    fi

    # Convert P99 to ms
    local p99_ms
    if [[ -n "$p99" ]]; then
        p99_ms=$(echo "$p99 * 1000 / 1" | bc)
    else
        p99_ms="N/A"
    fi

    echo "  Throughput: ${rps} req/s"
    echo "  P99 Latency: ${p99_ms}ms"
    echo "  Success Rate: ${success_rate}%"
    echo "  Total Requests: ${total}"

    # Write to results file
    echo "| ${name} | ${rps} | ${p99_ms}ms | ${success_rate}% | ${total} |" >> "$RESULTS_FILE"
}

# ============================================
# Initialize Results File
# ============================================
cat > "$RESULTS_FILE" << EOF
# Benchmark Results: ${IMPL^^} Implementation

**Date:** $(date)
**Target:** ${HOST}
**Duration:** ${DURATION}s per test
**Concurrency:** ${AGENTS} agents
**Implementation:** ${IMPL}

## Results

| Test | Throughput (req/s) | P99 Latency | Success Rate | Total Requests |
|------|-------------------|-------------|--------------|----------------|
EOF

# ============================================
# Run Benchmarks
# ============================================
wait_for_server

echo ""
echo "=== Phase 1: HTTP Layer ==="
run_benchmark "Health (no I/O)" "$HEALTH_ENDPOINT"
run_benchmark "Ready (DB check)" "$READY_ENDPOINT"

echo ""
echo "=== Phase 2: MCP Protocol ==="
MCP_BODY='{"jsonrpc":"2.0","method":"tools/list","params":{},"id":1}'
run_benchmark "MCP tools/list" "/mcp" "POST" "$MCP_BODY" "application/json"

# Setup for message tests
echo ""
echo "=== Phase 3: Setup Test Data ==="
curl -s -X POST "${HOST}/api/project/ensure" \
  -H "Content-Type: application/json" \
  -d '{"human_key": "benchmark-project"}' >/dev/null

curl -s -X POST "${HOST}/api/agent/register" \
  -H "Content-Type: application/json" \
  -d '{"project_slug":"benchmark-project","name":"sender","program":"bench","model":"test"}' >/dev/null 2>&1 || true

curl -s -X POST "${HOST}/api/agent/register" \
  -H "Content-Type: application/json" \
  -d '{"project_slug":"benchmark-project","name":"receiver","program":"bench","model":"test"}' >/dev/null 2>&1 || true

echo "Test data ready"

echo ""
echo "=== Phase 4: Message Operations ==="
MSG_BODY='{"project_slug":"benchmark-project","sender_name":"sender","recipient_names":["receiver"],"subject":"Bench","body_md":"Test"}'
run_benchmark "Message Send" "/api/message/send" "POST" "$MSG_BODY"

INBOX_BODY='{"project_slug":"benchmark-project","agent_name":"receiver"}'
run_benchmark "Inbox Fetch" "/api/inbox" "POST" "$INBOX_BODY"

# ============================================
# Add Summary
# ============================================
cat >> "$RESULTS_FILE" << EOF

## Environment

- Implementation: ${IMPL^^}
- Health Endpoint: ${HEALTH_ENDPOINT}
- Ready Endpoint: ${READY_ENDPOINT}
- Server: ${HOST}

## Analysis Notes

Add observations about performance characteristics here.

EOF

echo ""
echo "=============================================="
echo "Benchmark Complete!"
echo "=============================================="
echo "Results saved to: ${RESULTS_FILE}"
```

### 5.2 Running Both Implementations

```bash
# Make script executable
chmod +x scripts/unified_benchmark.sh

# ============================================
# Step 1: Benchmark Rust
# ============================================
# Terminal 1: Start Rust server
rm -rf data && mkdir -p data/archive
RATE_LIMIT_ENABLED=false ./target/release/mcp-server

# Terminal 2: Run benchmark
./scripts/unified_benchmark.sh --impl rust --duration 30 --agents 100

# ============================================
# Step 2: Benchmark Python
# ============================================
# Terminal 1: Stop Rust, Start Python
pkill -f mcp-server
cd ../temp_mcp_mail
rm -rf data && mkdir -p data
uv run python -m mcp_agent_mail.http --host 0.0.0.0 --port 8765

# Terminal 2: Run benchmark
cd ../mcp-agent-mail-rs
./scripts/unified_benchmark.sh --impl python --duration 30 --agents 100
```

---

## 6. Metrics Collection

### 6.1 Standard Metrics Matrix

| Metric | Unit | Collection Method | Target |
|--------|------|-------------------|--------|
| **Throughput** | req/s | hey output | Higher is better |
| **P50 Latency** | ms | hey histogram | < 10ms |
| **P99 Latency** | ms | hey histogram | < 100ms |
| **P999 Latency** | ms | hey histogram | < 500ms |
| **Error Rate** | % | HTTP status codes | < 0.1% |
| **Memory RSS** | MB | `ps` / `top` | No growth |
| **CPU Usage** | % | `top` / `htop` | Efficient |
| **File Descriptors** | count | `lsof` | < ulimit |

### 6.2 Database-Specific Metrics

| Metric | Rust (libsql) | Python (SQLAlchemy) |
|--------|---------------|---------------------|
| Connection pool size | N/A (embedded) | `pool_size=10` |
| Max overflow | N/A | `max_overflow=10` |
| WAL mode | Enabled | Enabled |
| Busy timeout | 5000ms | 30000ms |
| Page cache hits | PRAGMA stats | N/A |

### 6.3 Resource Monitoring Script

```bash
#!/usr/bin/env bash
# scripts/monitor_resources.sh

INTERVAL=5
LOG_FILE="benchmark_results/resource_monitor_$(date +%Y%m%d_%H%M%S).csv"

echo "timestamp,pid,rss_mb,vsz_mb,cpu_pct,fd_count" > "$LOG_FILE"

while true; do
    TIMESTAMP=$(date +%s)

    # Find server process
    PID=$(pgrep -f "mcp-server\|uvicorn\|python.*http" | head -1)

    if [[ -n "$PID" ]]; then
        # Get memory and CPU
        STATS=$(ps -p "$PID" -o rss=,vsz=,%cpu= 2>/dev/null)
        RSS_KB=$(echo "$STATS" | awk '{print $1}')
        VSZ_KB=$(echo "$STATS" | awk '{print $2}')
        CPU=$(echo "$STATS" | awk '{print $3}')

        RSS_MB=$((RSS_KB / 1024))
        VSZ_MB=$((VSZ_KB / 1024))

        # Count file descriptors
        FD_COUNT=$(ls /proc/"$PID"/fd 2>/dev/null | wc -l || echo "N/A")

        echo "${TIMESTAMP},${PID},${RSS_MB},${VSZ_MB},${CPU},${FD_COUNT}" >> "$LOG_FILE"
        echo "RSS: ${RSS_MB}MB | VSZ: ${VSZ_MB}MB | CPU: ${CPU}% | FDs: ${FD_COUNT}"
    fi

    sleep "$INTERVAL"
done
```

---

## 7. Analysis Framework

### 7.1 Comparison Table Template

```markdown
## Head-to-Head Comparison: [Test Name]

| Metric | Rust | Python | Ratio | Winner |
|--------|------|--------|-------|--------|
| Throughput | X req/s | Y req/s | X/Y | |
| P50 Latency | Xms | Yms | Y/X | |
| P99 Latency | Xms | Yms | Y/X | |
| Memory (RSS) | XMB | YMB | Y/X | |
| CPU Usage | X% | Y% | Y/X | |
```

### 7.2 Bottleneck Analysis Checklist

For each implementation, analyze:

- [ ] **HTTP Layer**: Framework overhead, connection handling
- [ ] **MCP Layer**: Transport instantiation, JSON-RPC parsing
- [ ] **Database Layer**: Connection pool, query optimization, WAL
- [ ] **Git Layer**: Commit frequency, index operations, lock contention
- [ ] **Serialization**: JSON encoding/decoding overhead
- [ ] **Memory**: Allocation patterns, GC pressure (Python)
- [ ] **Concurrency**: Thread/task scheduling, lock contention

### 7.3 Performance Ratio Interpretation

| Ratio | Interpretation |
|-------|----------------|
| > 10x | Fundamental architectural difference |
| 5-10x | Significant optimization opportunity |
| 2-5x | Language/runtime overhead |
| 1-2x | Implementation detail difference |
| ~1x | Parity (database/IO bound) |

---

## 8. Automation Scripts

### 8.1 Full Comparison Suite

```bash
#!/usr/bin/env bash
# scripts/full_comparison.sh
# Runs complete benchmark suite for both implementations

set -euo pipefail

RESULTS_DIR="benchmark_results/comparison_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$RESULTS_DIR"

echo "Starting full benchmark comparison..."
echo "Results directory: $RESULTS_DIR"

# ============================================
# Phase 1: Rust Benchmarks
# ============================================
echo ""
echo "===== RUST IMPLEMENTATION ====="

# Start Rust server
pkill -f "mcp-server\|uvicorn\|python.*http" 2>/dev/null || true
sleep 2
rm -rf data && mkdir -p data/archive
RATE_LIMIT_ENABLED=false ./target/release/mcp-server &
RUST_PID=$!
sleep 3

# Run benchmarks
for agents in 10 50 100 200; do
    echo "Testing Rust with $agents agents..."
    ./scripts/unified_benchmark.sh --impl rust --agents $agents --duration 10 \
        --output "$RESULTS_DIR/rust_${agents}_agents.md"
done

kill $RUST_PID 2>/dev/null || true
sleep 2

# ============================================
# Phase 2: Python Benchmarks
# ============================================
echo ""
echo "===== PYTHON IMPLEMENTATION ====="

# Start Python server
cd ../temp_mcp_mail
rm -rf data && mkdir -p data
uv run python -m mcp_agent_mail.http --host 0.0.0.0 --port 8765 &
PYTHON_PID=$!
cd ../mcp-agent-mail-rs
sleep 5

# Run benchmarks
for agents in 10 50 100 200; do
    echo "Testing Python with $agents agents..."
    ./scripts/unified_benchmark.sh --impl python --agents $agents --duration 10 \
        --output "$RESULTS_DIR/python_${agents}_agents.md"
done

kill $PYTHON_PID 2>/dev/null || true

# ============================================
# Generate Summary
# ============================================
echo ""
echo "Generating comparison summary..."

cat > "$RESULTS_DIR/SUMMARY.md" << EOF
# Benchmark Comparison Summary

**Generated:** $(date)
**Concurrency Levels:** 10, 50, 100, 200 agents
**Duration:** 10s per test

## Quick Comparison

See individual result files for detailed metrics.

## Files

$(ls -1 "$RESULTS_DIR"/*.md | sed 's/^/- /')
EOF

echo ""
echo "===== COMPLETE ====="
echo "Results saved to: $RESULTS_DIR"
```

### 8.2 Quick Sanity Check

```bash
#!/usr/bin/env bash
# scripts/quick_check.sh
# Fast sanity check for both implementations

echo "Quick benchmark check (5s each)..."

# Rust
echo "=== RUST ==="
hey -c 100 -z 5s http://127.0.0.1:8765/health 2>&1 | grep -E "(Requests/sec|99%)"

# Python (if running on different port or after switching)
echo "=== PYTHON ==="
hey -c 100 -z 5s http://127.0.0.1:8765/health/liveness 2>&1 | grep -E "(Requests/sec|99%)"
```

---

## 9. Success Criteria

### 9.1 Minimum Viable Performance

| Metric | Rust Target | Python Target |
|--------|-------------|---------------|
| Health throughput | > 50,000 req/s | > 5,000 req/s |
| MCP throughput | > 50,000 req/s | > 200 req/s |
| Message send | > 1,000 msg/s | > 100 msg/s |
| P99 latency | < 20ms | < 200ms |
| Error rate | < 0.01% | < 0.1% |

### 9.2 Scalability Requirements

| Dimension | Target |
|-----------|--------|
| Concurrent agents | 100+ sustained |
| Messages in DB | 1M+ |
| Memory growth | < 10MB/hour under load |
| Uptime | 24h+ soak test |

---

## 10. Appendix

### A. Tool Installation

```bash
# hey (HTTP load generator)
brew install hey           # macOS
go install github.com/rakyll/hey@latest  # Go

# k6 (advanced load testing)
brew install k6            # macOS
# or download from https://k6.io/

# wrk (high-performance HTTP benchmark)
brew install wrk           # macOS

# hyperfine (CLI benchmark)
brew install hyperfine     # macOS
cargo install hyperfine    # Rust
```

### B. Common Issues

| Issue | Rust | Python |
|-------|------|--------|
| Port in use | `pkill -f mcp-server` | `pkill -f uvicorn` |
| Rate limiting | `RATE_LIMIT_ENABLED=false` | Edit config or code |
| Connection refused | Check server startup logs | Check import errors |
| Timeout errors | Increase `-t` in hey | Check async/await |

### C. References

- [hey Documentation](https://github.com/rakyll/hey)
- [k6 Documentation](https://k6.io/docs/)
- [Criterion.rs Book](https://bheisler.github.io/criterion.rs/book/)
- [Axum Performance](https://github.com/tokio-rs/axum)
- [FastAPI Performance](https://fastapi.tiangolo.com/benchmarks/)

---

*Document Version: 1.0*
*Created: 2025-12-16*
*Author: Claude Code*
