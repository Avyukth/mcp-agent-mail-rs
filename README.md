# MCP Agent Mail (Rust)

> "It's like Gmail for your coding agents!"

A high-performance Rust implementation of a mail-like coordination layer for AI coding agents, exposed as both REST API and MCP (Model Context Protocol) server. Enables asynchronous communication between multiple agents working on shared codebases with full audit trails.

**Ported from**: [mcp_agent_mail (Python)](https://github.com/Dicklesworthstone/mcp_agent_mail)

## Why This Exists

Modern projects often run multiple coding agents simultaneously (backend, frontend, scripts, infra). Without coordination, agents:

- Overwrite each other's edits or panic on unexpected diffs
- Miss critical context from parallel workstreams
- Require humans to "liaison" messages across tools

MCP Agent Mail provides:

- **Agent Identity**: Memorable adjective+noun names (BlueMountain, GreenCastle)
- **Messaging**: GitHub-Flavored Markdown messages with threading and attachments
- **File Reservations**: Advisory locks to prevent edit conflicts
- **Contact Management**: Explicit approval for cross-project messaging
- **Searchable Archives**: FTS5 full-text search across message bodies
- **Git-Backed Audit Trail**: All messages persisted for human review

## Quick Start

### Prerequisites

- **Rust** 1.83+ with cargo
- **Bun** (for frontend development)
- **just** (optional, recommended command runner)

```bash
# Install just (optional but recommended)
cargo install just

# Install bun (if not already installed)
curl -fsSL https://bun.sh/install | bash
```

### Development Setup

```bash
# Clone the repository
git clone <repository-url>
cd mcp-agent-mail-rs

# Build all Rust components
cargo build --workspace

# Install frontend dependencies
cd crates/services/web-ui && bun install && cd ../../..

# Run development servers (API + Web UI)
just dev
# Or using make:
make dev
```

Development servers:
- **API Server**: http://localhost:8000
- **Web UI**: http://localhost:5173 (with hot reload)

### Production Build

```bash
# Build everything for production
just prod
# Or:
make build-prod

# Run production server
make run-prod
```

### Docker Deployment

```bash
# Build and run with Docker Compose
docker-compose up -d

# Health check
curl http://localhost:8000/health
```

## Architecture

```
mcp-agent-mail-rs/
├── crates/
│   ├── libs/
│   │   └── lib-core/           # Domain logic, BMC pattern, storage
│   │       ├── src/model/      # Entities (Agent, Message, Project, FileReservation)
│   │       ├── src/store/      # Database (libsql) + Git (git2) storage
│   │       └── tests/          # Integration tests
│   └── services/
│       ├── mcp-server/         # Axum REST API server
│       ├── mcp-stdio/          # MCP protocol server (stdio + SSE)
│       ├── mcp-cli/            # CLI for testing
│       └── web-ui/             # SvelteKit frontend
├── migrations/                 # SQLite schema with FTS5
├── data/                       # Runtime data (SQLite DB, Git archive)
└── docs/                       # Project documentation
```

### Tech Stack

| Layer | Technology |
|-------|------------|
| **Backend** | Rust, Axum 0.8, tokio |
| **Database** | libsql (SQLite) with FTS5 full-text search |
| **Storage** | git2 for audit trail |
| **Protocol** | MCP via rmcp SDK |
| **Frontend** | SvelteKit 2, Svelte 5, TailwindCSS |
| **Package Manager** | Cargo (Rust), Bun (JS) |
| **Metrics** | Prometheus (metrics-exporter-prometheus) |

### Design Patterns

#### Backend Model Controller (BMC) Pattern

Separates concerns for each entity:

```rust
// Entity struct (database row)
pub struct Agent {
    pub id: i64,
    pub name: String,
    // ...
}

// Creation input
pub struct AgentForCreate {
    pub name: String,
    // ...
}

// Business logic
pub struct AgentBmc;
impl AgentBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, data: AgentForCreate) -> Result<i64>;
    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Agent>;
    // ...
}
```

#### Dual Persistence

All data stored in both:
1. **SQLite** (libsql): Fast queries, FTS5 search, transactions
2. **Git Repository**: Human-readable audit trail

```
data/archive/projects/{slug}/
├── agents/{name}/
│   ├── profile.json
│   ├── inbox/YYYY/MM/{message}.md
│   └── outbox/YYYY/MM/{message}.md
└── messages/YYYY/MM/{timestamp}__{subject}__{id}.md
```

## API Reference

### Health & Monitoring

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check with uptime |
| `/ready` | GET | Readiness probe (DB connectivity) |
| `/metrics` | GET | Prometheus metrics |

### Core Operations

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/project/ensure` | POST | Create or get existing project |
| `/api/projects` | GET | List all projects |
| `/api/projects/{slug}/agents` | GET | List agents for project |
| `/api/agent/register` | POST | Register new agent |
| `/api/agent/whois` | POST | Lookup agent by name |
| `/api/message/send` | POST | Send a message |
| `/api/message/reply` | POST | Reply to a message |
| `/api/inbox` | POST | List messages in inbox |
| `/api/messages/{id}` | GET | Get single message |
| `/api/messages/search` | POST | Full-text search messages |

### File Reservations

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/file_reservations/paths` | POST | Reserve file paths |
| `/api/file_reservations/list` | POST | List active reservations |
| `/api/file_reservations/release` | POST | Release reservations |
| `/api/file_reservations/renew` | POST | Extend TTL |
| `/api/file_reservations/force_release` | POST | Force release (admin) |

### MCP Protocol (stdio)

```bash
# Run MCP server for Claude Desktop integration
cargo run -p mcp-stdio -- serve

# Or with SSE transport
cargo run -p mcp-stdio -- serve --transport sse --port 3000

# List all available tools
cargo run -p mcp-stdio -- tools

# Export JSON schema
cargo run -p mcp-stdio -- schema
```

## Database Schema

SQLite with FTS5 full-text search. Key tables:

| Table | Description |
|-------|-------------|
| `projects` | Project registry (slug, human_key) |
| `agents` | Agent profiles per project |
| `messages` | Message content with threading |
| `message_recipients` | To/CC/BCC with read/ack tracking |
| `messages_fts` | FTS5 index for search |
| `file_reservations` | Advisory file locks |
| `agent_links` | Cross-project contact approval |
| `build_slots` | Exclusive build resource locks |
| `macros` | Reusable workflow definitions |

See `migrations/001_initial_schema.sql` for full schema.

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | 8000 | API server port |
| `RUST_LOG` | info | Log level (debug, info, warn, error) |
| `LOG_FORMAT` | text | Log format (text, json) |
| `DATABASE_URL` | data/mcp_agent_mail.db | SQLite database path |

### Project Structure

```bash
# View project status with beads
bd status

# Check ready work
bd ready --json
```

## Development Commands

### Using just (recommended)

```bash
just dev        # Run API + Web UI with hot reload
just api        # Run API server only
just web        # Run Web UI only
just mcp        # Run MCP stdio server
just build      # Build debug
just release    # Build release
just test       # Run all tests
just lint       # Run clippy
just fmt        # Format code
just tools      # List MCP tools
just schema     # Export JSON schema
just clean      # Clean build artifacts
```

### Using make

```bash
make dev        # Run API + Web UI
make dev-api    # Run API server only
make dev-web    # Run Web UI only
make dev-mcp    # Run MCP stdio server
make build      # Build debug
make build-release  # Build release
make test       # Run all tests
make lint       # Run clippy
make fmt        # Format code
make tools      # List MCP tools
make schema     # Export JSON schema
make clean      # Clean all artifacts
```

### Using cargo directly

```bash
# Build
cargo build --workspace
cargo build --workspace --release

# Test
cargo test -p lib-core --test integration -- --test-threads=1
cargo test -p mcp-stdio --test integration -- --test-threads=1

# Lint
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all

# Run
cargo run -p mcp-server
cargo run -p mcp-stdio -- serve
```

## Best Practices

### Code Quality

- **No `unwrap()` in production code** - Use proper error handling with `?` and `thiserror`
- **Run clippy before commits** - `cargo clippy --workspace -- -D warnings`
- **Format code** - `cargo fmt --all`
- **Write integration tests** - See `crates/libs/lib-core/tests/`

### Error Handling

```rust
// Use thiserror for domain errors
#[derive(Debug, Error)]
pub enum Error {
    #[error("Agent not found: {0}")]
    AgentNotFound(String),

    #[error("Libsql Error: {0}")]
    Libsql(#[from] libsql::Error),
}

// Return Result<T> from all functions
pub type Result<T> = core::result::Result<T, Error>;
```

### Git Commits

```bash
# Commit with beads task reference
git commit -m "feat: implement X (closes bd-Y)"

# Include .beads/issues.jsonl in commits
git add .beads/issues.jsonl
```

### Task Tracking

This project uses **beads (`bd`)** for issue tracking:

```bash
# Find ready work
bd ready --json

# Claim a task
bd update <id> --status in_progress

# Complete a task
bd close <id> --reason "Implemented X"

# Create new issues
bd create "Bug: X" -t bug -p 1
```

## Project Status

| Phase | Status | Description |
|-------|--------|-------------|
| 1 | COMPLETE | Core Architecture (BMC, storage) |
| 1.5 | COMPLETE | API Layer (Axum REST endpoints) |
| 2 | COMPLETE | SvelteKit Frontend |
| 3 | IN PROGRESS | Full Feature Parity (28 MCP tools) |
| 4 | PLANNED | MCP Protocol Integration |
| 5 | PLANNED | Production Hardening |

See `docs/PROJECT_PLAN.md` for detailed task breakdown.

## Integration with AI Agents

### Claude Desktop (MCP)

Add to `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "agent-mail": {
      "command": "/path/to/mcp-agent-mail-rs/target/release/mcp-stdio",
      "args": ["serve"]
    }
  }
}
```

### Codex CLI

AGENTS.md is automatically read. Start with:

```bash
cd /path/to/mcp-agent-mail-rs
codex
# Then: "Run bd ready --json and start the first task"
```

### Gemini CLI

```bash
# Symlink AGENTS.md
ln -s ../AGENTS.md .gemini/GEMINI.md

# Run
gemini
```

## Typical Workflow

### For Agents

```bash
# 1. Register identity
curl -X POST http://localhost:8000/api/agent/register \
  -H "Content-Type: application/json" \
  -d '{"project_key": "/path/to/project", "name": "BlueMountain", "program": "claude", "model": "opus"}'

# 2. Reserve files before editing
curl -X POST http://localhost:8000/api/file_reservations/paths \
  -H "Content-Type: application/json" \
  -d '{"project_slug": "my-project", "agent_name": "BlueMountain", "paths": ["src/**"], "ttl_seconds": 3600}'

# 3. Send progress message
curl -X POST http://localhost:8000/api/message/send \
  -H "Content-Type: application/json" \
  -d '{"project_slug": "my-project", "from_agent": "BlueMountain", "to_agents": ["GreenCastle"], "subject": "Starting refactor", "body_md": "Working on auth module..."}'

# 4. Release reservation when done
curl -X POST http://localhost:8000/api/file_reservations/release \
  -H "Content-Type: application/json" \
  -d '{"project_slug": "my-project", "agent_name": "BlueMountain", "paths": ["src/**"]}'
```

### For Humans

1. **Web UI**: http://localhost:8000/mail (when integrated) or http://localhost:5173 (dev)
2. **CLI**: `cargo run -p mcp-cli -- inbox --project my-project --agent BlueMountain`
3. **Git**: Browse `data/archive/` for full audit trail

## References

- [Python Original](https://github.com/Dicklesworthstone/mcp_agent_mail) - Source implementation
- [MCP Tools Reference](https://glama.ai/mcp/servers/@Dicklesworthstone/mcp_agent_mail) - 28 MCP tools specification
- [Beads Issue Tracker](https://github.com/steveyegge/beads) - Task tracking via `bd` CLI
- [MCP Protocol](https://modelcontextprotocol.io) - Model Context Protocol specification

## License

See LICENSE file for details.

---

Built with Rust for memory safety, performance, and reliability.
