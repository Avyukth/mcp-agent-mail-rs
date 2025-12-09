# MCP Agent Mail - Makefile
# Unified build and run commands for the Rust implementation

.PHONY: all build build-release build-web dev run run-all clean test help

# Default target
all: build

# ============================================================================
# Build Commands
# ============================================================================

## Build all components (debug)
build:
	@echo "🔨 Building Rust components..."
	cargo build --workspace
	@echo "✅ Build complete"

## Build all components (release)
build-release:
	@echo "🔨 Building Rust components (release)..."
	cargo build --workspace --release
	@echo "✅ Release build complete"

## Build web UI for production
build-web:
	@echo "🌐 Building SvelteKit frontend..."
	cd crates/services/web-ui && bun install && bun run build
	@echo "✅ Frontend build complete"

## Build everything for production
build-prod: build-release build-web
	@echo "🎉 Production build complete"

# ============================================================================
# Development Commands
# ============================================================================

## Run API server (development)
dev-api:
	@echo "🚀 Starting API server on http://localhost:8000..."
	cargo run -p mcp-server

## Run web UI (development with hot reload)
dev-web:
	@echo "🌐 Starting SvelteKit dev server on http://localhost:5173..."
	cd crates/services/web-ui && bun run dev

## Run MCP stdio server
dev-mcp:
	@echo "📨 Starting MCP stdio server..."
	cargo run -p mcp-stdio -- serve

## Run all services in development (parallel)
dev:
	@echo "🚀 Starting all development servers..."
	@echo "   API: http://localhost:8000"
	@echo "   Web: http://localhost:5173"
	$(MAKE) -j2 dev-api dev-web

# ============================================================================
# Production Commands
# ============================================================================

## Run API server (release mode)
run:
	@echo "🚀 Starting API server (release) on http://localhost:8000..."
	cargo run -p mcp-server --release

## Run integrated server with built frontend
run-prod: build-prod
	@echo "🚀 Starting production server on http://localhost:8000..."
	@echo "   API:    http://localhost:8000/api/*"
	@echo "   Web UI: http://localhost:8000/mail"
	cargo run -p mcp-server --release

# ============================================================================
# Testing
# ============================================================================

## Run all tests
test:
	@echo "🧪 Running all tests..."
	cargo test -p lib-core --test integration -- --test-threads=1
	cargo test -p mcp-stdio --test integration -- --test-threads=1
	@echo "✅ All tests passed (26 total)"

## Run tests with coverage
test-coverage:
	@echo "🧪 Running tests with coverage..."
	cargo tarpaulin --workspace --out Html

## Run clippy lints
lint:
	@echo "🔍 Running clippy..."
	cargo clippy --workspace --all-targets -- -D warnings

## Format code
fmt:
	@echo "✨ Formatting code..."
	cargo fmt --all

## Check formatting without modifying
fmt-check:
	@echo "✨ Checking code format..."
	cargo fmt --all -- --check

# ============================================================================
# Database
# ============================================================================

## Initialize/reset database
db-reset:
	@echo "🗄️  Resetting database..."
	rm -f data/storage.db
	@echo "✅ Database reset (will be recreated on next run)"

## Show database path
db-info:
	@echo "📂 Database location: data/storage.db"
	@ls -lh data/storage.db 2>/dev/null || echo "   (not created yet)"

# ============================================================================
# Utilities
# ============================================================================

## List all MCP tools
tools:
	@cargo run -p mcp-stdio -- tools

## Export JSON schema for all tools
schema:
	@cargo run -p mcp-stdio -- schema

## Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	rm -rf crates/services/web-ui/.svelte-kit
	rm -rf crates/services/web-ui/build
	rm -rf crates/services/web-ui/node_modules
	@echo "✅ Clean complete"

## Check beads ready work
ready:
	@bd ready --json

## Show help
help:
	@echo "MCP Agent Mail - Rust Implementation"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Build:"
	@echo "  build        Build all Rust components (debug)"
	@echo "  build-release Build all Rust components (release)"
	@echo "  build-web    Build SvelteKit frontend for production"
	@echo "  build-prod   Build everything for production"
	@echo ""
	@echo "Development (with hot reload):"
	@echo "  dev          Run API + Web UI in parallel"
	@echo "  dev-api      Run API server only"
	@echo "  dev-web      Run SvelteKit dev server only"
	@echo "  dev-mcp      Run MCP stdio server"
	@echo ""
	@echo "Production:"
	@echo "  run          Run API server (release mode)"
	@echo "  run-prod     Build and run production server"
	@echo ""
	@echo "Testing:"
	@echo "  test         Run all integration tests"
	@echo "  lint         Run clippy lints"
	@echo "  fmt          Format code"
	@echo ""
	@echo "Utilities:"
	@echo "  tools        List all MCP tools"
	@echo "  schema       Export JSON schema for tools"
	@echo "  clean        Remove build artifacts"
	@echo "  ready        Show beads ready work"
	@echo "  help         Show this help message"
