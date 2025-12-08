# GitHub Copilot Instructions: MCP Agent Mail (Rust)

## Project Overview

**mcp-agent-mail-rs** is a Rust implementation of "Gmail for coding agents" - an MCP-compliant mail server for AI agent communication. We're translating from Python using Depyler-assisted transpilation.

**Tech Stack**: Rust (Axum, libsql, git2) + SvelteKit + TailwindCSS + Bun

## Issue Tracking with bd (beads)

**CRITICAL**: This project uses **bd (beads)** for ALL task tracking. Do NOT create markdown TODO lists.

### Essential Commands

```bash
# Find work with no blockers
bd ready --json

# Create issues
bd create "Title" -t task|bug|feature -p 0-4 --json
bd create "Subtask" --parent <epic-id> --json

# Update status
bd update <id> --status in_progress --json

# Complete work
bd close <id> --reason "Done" --json
```

### Current Phase: Phase 2 - SvelteKit Frontend

Ready tasks (run `bd ready --json`):
- `mcp-agent-mail-rs-k43.1`: Initialize SvelteKit project
- `mcp-agent-mail-rs-k43.4`: Configure adapter-static

### Workflow

1. **Check ready work**: `bd ready --json`
2. **Claim task**: `bd update <id> --status in_progress --json`
3. **Implement**: Write code, tests
4. **Found new work?**: `bd create "Bug" --deps discovered-from:<id> --json`
5. **Complete**: `bd close <id> --reason "Done" --json`
6. **Commit**: `git add -A && git commit -m "feat: X (closes bd-Y)"`

## Project Structure

```
mcp-agent-mail-rs/
├── crates/
│   ├── libs/lib-core/      # Domain logic (Project, Agent, Message BMCs)
│   └── services/
│       ├── mcp-server/     # Axum REST API (port 8000)
│       ├── mcp-cli/        # CLI for testing
│       └── web-ui/         # SvelteKit frontend (Phase 2)
├── migrations/             # SQL schema (libsql)
├── .beads/                 # Issue tracker database
└── AGENTS.md               # Full agent instructions
```

## Coding Guidelines

### Rust
- Use `thiserror` for error types
- Follow BMC (Backend Model Controller) pattern
- No `unwrap()` in production code
- Run `cargo clippy` before committing

### SvelteKit (Phase 2)
- Use Bun as package manager
- TailwindCSS for styling
- adapter-static for embedding in Rust binary
- API calls to `http://localhost:8000/api/*`

## Quality Gates

Before completing any task:
- [ ] `cargo build` succeeds
- [ ] `cargo clippy` has no warnings
- [ ] `cargo test` passes
- [ ] Documentation updated if needed

## Important Rules

- ✅ Use `bd` for ALL task tracking
- ✅ Always use `--json` flag for parsing
- ✅ Commit `.beads/issues.jsonl` with code
- ❌ Do NOT create markdown TODO lists
- ❌ Do NOT skip claiming issues

## References

- [AGENTS.md](../AGENTS.md) - Full agent workflow guide
- [PROJECT_PLAN.md](../docs/PROJECT_PLAN.md) - Detailed task breakdown
- [llms.txt](../llms.txt) - LLM-friendly overview
