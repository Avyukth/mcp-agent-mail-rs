#!/usr/bin/env bash
# Mouchak Mail - Quick Start Script
# Builds and runs the development servers

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Mouchak Mail - Development Server${NC}"
echo ""

# Check for dependencies
command -v cargo >/dev/null 2>&1 || { echo "cargo is required but not installed. Aborting."; exit 1; }
command -v bun >/dev/null 2>&1 || { echo "bun is required but not installed. Aborting."; exit 1; }

# Build if needed
if [ ! -f "target/debug/mcp-server" ]; then
    echo -e "${GREEN}Building Rust components...${NC}"
    cargo build --workspace
fi

# Install web dependencies if needed
if [ ! -d "crates/services/web-ui/node_modules" ]; then
    echo -e "${GREEN}Installing web dependencies...${NC}"
    cd crates/services/web-ui && bun install && cd "$PROJECT_ROOT"
fi

echo ""
echo -e "${GREEN}Starting servers...${NC}"
echo "  API:     http://localhost:8000"
echo "  Web UI:  http://localhost:5173"
echo "  Health:  http://localhost:8000/health"
echo ""
echo "Press Ctrl+C to stop all servers"
echo ""

# Run both servers in parallel
trap 'kill 0' EXIT

# Start API server in background
cargo run -p mcp-server 2>&1 | sed 's/^/[API] /' &

# Wait for API to be ready
sleep 2

# Start web UI
cd crates/services/web-ui && bun run dev 2>&1 | sed 's/^/[WEB] /' &

# Wait for all background processes
wait
