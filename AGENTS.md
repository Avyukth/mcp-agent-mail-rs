# Agent Instructions: MCP Agent Mail (Rust)

> This document provides instructions for AI coding agents (Claude, Gemini, Codex, GPT, etc.) working on this project.

## Project Overview

**Goal**: Translate Python `mcp_agent_mail` ("Gmail for coding agents") to idiomatic Rust with a SvelteKit frontend.

**Strategy**: Depyler-assisted transpilation + manual refinement

**Current Phase**: Phase 1.5 complete (API Layer) → Phase 2 (SvelteKit Frontend) is next

## Issue Tracking with bd (beads)

**CRITICAL**: This project uses **bd (beads)** for ALL issue tracking. Do NOT use markdown TODOs, task lists, or other tracking methods.

### Why bd?

- Dependency-aware: Track blockers and relationships between issues
- Git-friendly: Auto-syncs to JSONL for version control
- Agent-optimized: JSON output, ready work detection, discovered-from links
- Multi-agent: All agents share the same issue database via git

### Quick Reference

```bash
# Find ready work (no blockers)
bd ready --json

# Create new issues
bd create "Issue title" -t bug|feature|task -p 0-4 --json
bd create "Subtask" --parent <epic-id> --json

# Claim and update
bd update <id> --status in_progress --json

# Complete work
bd close <id> --reason "Completed" --json

# View all issues
bd list --json
```

### Issue Types

| Type | Use For |
|------|---------|
| `epic` | Large features with subtasks (phases) |
| `feature` | New functionality |
| `task` | Work items (implementation, tests, docs) |
| `bug` | Something broken |
| `chore` | Maintenance (deps, tooling) |

### Priorities

| Priority | Meaning |
|----------|---------|
| 0 | Critical (security, data loss) |
| 1 | High (major features) |
| 2 | Medium (default) |
| 3 | Low (polish) |
| 4 | Backlog (future) |

## Current Project Phases (Epics)

Run `bd list -t epic --json` to see all phases:

| Phase | Epic ID | Status | Description |
|-------|---------|--------|-------------|
| 1 | bd-* | done | Core Architecture |
| 1.5 | bd-* | done | API Layer (Axum) |
| 2 | bd-* | todo | SvelteKit Frontend |
| 3 | bd-* | todo | Full Feature Parity (46 MCP Tools) |
| 4 | bd-* | todo | MCP Protocol Integration |
| 5 | bd-* | todo | Production Hardening |

## Workflow for AI Agents

### Starting a Session

```bash
# 1. Check for ready work
bd ready --json

# 2. Pick an issue and claim it
bd update <id> --status in_progress --json

# 3. Read the related epic for context
bd show <epic-id> --json
```

### During Development

```bash
# Found a bug or TODO?
bd create "Bug: missing validation" -t bug -p 1 --deps discovered-from:<current-id> --json

# Need to break down work?
bd create "Subtask: implement X" --parent <epic-id> --json
```

### Ending a Session

```bash
# 1. Complete finished work
bd close <id> --reason "Implemented feature X" --json

# 2. Update partially done work
bd update <id> --status in_progress --json
bd comment <id> "Progress: completed A, B remains"

# 3. Ensure changes are synced
# bd auto-syncs, but verify:
ls -la .beads/issues.jsonl

# 4. Commit together with code changes
git add .beads/issues.jsonl
git commit -m "feat: implement X (closes bd-123)"
```

## Tech Stack

| Layer | Technology |
|-------|------------|
| Backend | Rust, Axum 0.6, libsql, git2, tokio |
| Frontend | SvelteKit, TailwindCSS, Bun |
| Database | SQLite (libsql) with FTS5 |
| Storage | Git-backed mailbox (git2) |
| Protocol | MCP (Model Context Protocol) |

## Key Directories

```
mcp-agent-mail-rs/
├── crates/
│   ├── libs/lib-core/      # Domain logic, BMC pattern
│   └── services/
│       ├── mcp-server/     # Axum REST API
│       ├── mcp-cli/        # CLI for testing
│       └── web-ui/         # SvelteKit (Phase 2)
├── migrations/             # SQL schema
├── docs/
│   └── PROJECT_PLAN.md     # Detailed task breakdown
├── .beads/                 # Issue tracker
└── llms.txt                # LLM-friendly overview
```

## Quality Gates

Before marking work complete:

- [ ] Code compiles: `cargo build`
- [ ] No warnings: `cargo clippy`
- [ ] Tests pass: `cargo test`
- [ ] No `unwrap()` in production code
- [ ] Update docs if behavior changed

## Triggering Other Agents

### For Gemini CLI

Gemini CLI reads `GEMINI.md` and supports MCP servers.

**Setup:**
```bash
# Create .gemini directory
mkdir -p .gemini

# Symlink AGENTS.md as GEMINI.md (or copy)
ln -s ../AGENTS.md .gemini/GEMINI.md

# Or use environment variable for system instructions
export GEMINI_SYSTEM_MD=true
# Then create .gemini/system.md with instructions
```

**Run:**
```bash
cd /path/to/mcp-agent-mail-rs
gemini

# Then tell it:
# "Run bd ready --json and pick up the highest priority unblocked task"
# "Work on mcp-agent-mail-rs-k43.1 - Initialize SvelteKit project"
```

**MCP Integration (recommended):**
```bash
# Install beads MCP server
pip install beads-mcp

# Configure in ~/.gemini/settings.json:
# { "mcpServers": { "beads": { "command": "beads-mcp" } } }
```

### For OpenAI Codex CLI

Codex CLI automatically reads `AGENTS.md` files in precedence order.

**Setup:**
```bash
# AGENTS.md already exists - Codex will find it automatically
# For global instructions, also add to ~/.codex/AGENTS.md
```

**Run:**
```bash
cd /path/to/mcp-agent-mail-rs
codex

# Or non-interactively:
codex exec "Run bd ready --json, claim the first task, and implement it"
```

**With Agents SDK (advanced):**
```bash
# Codex can be exposed as MCP server for orchestration
# See: https://developers.openai.com/codex/guides/agents-sdk/
```

### For Claude Code

Claude Code automatically reads `CLAUDE.md` and `AGENTS.md`.

**Setup:**
```bash
# Create CLAUDE.md that references AGENTS.md
echo "See AGENTS.md for project-specific instructions." > CLAUDE.md
```

**Run:**
```bash
cd /path/to/mcp-agent-mail-rs
claude

# Then:
# "Run bd ready --json and start the first task"
# "Work on Phase 2 SvelteKit setup"
```

### For Any Agent (Universal Protocol)

Any AI coding agent should follow this workflow:

```bash
# 1. ORIENT - Understand the project
cat AGENTS.md        # Agent instructions
cat llms.txt         # Project overview
bd ready --json      # Available work

# 2. CLAIM - Pick and claim a task
bd update <id> --status in_progress --json

# 3. WORK - Implement the task
# ... write code, tests, docs ...

# 4. DISCOVER - File new issues found during work
bd create "Bug: X" -t bug --deps discovered-from:<id> --json

# 5. COMPLETE - Close the task
bd close <id> --reason "Implemented X" --json

# 6. SYNC - Commit everything together
git add -A
git commit -m "feat: implement X (closes bd-Y)"
```

### Multi-Agent Handoff Protocol

When handing off work between agents:

1. **Outgoing agent**: Update issue with progress note
   ```bash
   bd comment <id> "Progress: completed A and B, remaining: C and D"
   bd update <id> --status open --json  # Release claim
   ```

2. **Incoming agent**: Check recent activity
   ```bash
   bd show <id> --json           # See full issue with comments
   bd list --status open --json  # See all open work
   bd ready --json               # See unblocked work
   ```

3. **Coordination via git**:
   ```bash
   git pull                      # Get latest .beads/issues.jsonl
   bd ready --json               # Beads auto-imports from JSONL
   ```

## Important Rules

- ✅ Use `bd` for ALL task tracking
- ✅ Always use `--json` flag for programmatic parsing
- ✅ Link discovered work with `discovered-from` dependencies
- ✅ Check `bd ready` before asking "what should I work on?"
- ✅ Commit `.beads/issues.jsonl` with code changes
- ❌ Do NOT create markdown TODO lists
- ❌ Do NOT use external issue trackers
- ❌ Do NOT skip claiming issues before working

## References

- [Python Source](https://github.com/Dicklesworthstone/mcp_agent_mail)
- [Depyler Transpiler](https://github.com/paiml/depyler)
- [Beads Issue Tracker](https://github.com/steveyegge/beads)
- [MCP Protocol](https://modelcontextprotocol.io)
- [Project Plan](docs/PROJECT_PLAN.md)
