# AGENTS.md — Universal Operating Manual for AI Coding Agents

> **Quick Start**: `cm context "<task>"` → `bd ready` → work → `bd sync`

---

## ⛔ LAYER 0: INVIOLABLE SAFETY RULES

### Rule 1: NO FILE DELETION WITHOUT PERMISSION
Never delete files without explicit written permission. Applies to ALL files.

### Rule 2: NO DESTRUCTIVE COMMANDS
Forbidden: `git reset --hard`, `git clean -fd`, `rm -rf`, `git push`, `git push --force`

**Protocol**: Safer alternatives first → Explicit plan → Wait for confirmation → Document → Refuse if ambiguous

### Rule 3: PROTECT CONFIGURATION FILES
Never overwrite `.env`, lock files (`Cargo.lock`, `package-lock.json`), or database files.

---

## 🧠 LAYER 1: UNIVERSAL TOOLING

### Quick Reference

| Task | Tool | Command |
|------|------|---------|
| Start task | cm | `cm context "<task>"` |
| Find work | bd | `bd ready` |
| Search history | cass | `cass search "query" --robot` |
| Quality gates | pmat | `pmat analyze tdg` |
| Multi-agent coord | MCP Agent Mail | MCP tools or `am` CLI |

### 📚 cm (Context Memory)
```bash
cm context "<task>"     # Hydrate context (START HERE)
cm doctor              # Health check
cm mark <id> --helpful/--harmful  # Feedback
```

### 🔎 cass (Session Search)
⚠️ NEVER run bare `cass` — always use `--robot` or `--json`
```bash
cass health --json              # Pre-flight check
cass search "query" --robot --limit 5
cass view /path -n 42 --json    # View specific result
```

### 📋 bd (Beads Issue Tracking)
```bash
bd ready                        # Find unblocked work
bd create "Title" --description="..." -t task -p 2
bd update <id> --status in_progress
bd close <id> --reason "Done"
bd sync                         # CRITICAL: Run at session end
```

**Types**: bug, feature, task, epic, chore
**Priorities**: 0=critical, 1=high, 2=medium, 3=low, 4=backlog
**Dependencies**: `bd dep add <issue> <depends-on>` (issue NEEDS depends-on)

### 📊 bv (Graph Analysis)
```bash
bv --robot-insights    # Graph metrics
bv --robot-plan        # Execution plan
```

### 🤖 vc (AI Executor)
Orchestrates agents with supervision and quality gates. Claims from bd, spawns agents, creates follow-on work.
```bash
vc run       # Start executor
vc status    # View state
```

### 📊 pmat (Quality Analysis)
```bash
pmat analyze tdg                    # Technical debt grade
pmat rust-project-score             # Repository health
pmat mutate --target src/ --threshold 85
```

### 🔍 Code Search
- **Structural** (refactoring): `ast-grep run -l Rust -p 'pattern'`
- **Fast text**: `rg "pattern" -t rust`

### 🤝 MCP Agent Mail

**Server**: `http://localhost:8765` | Start: `am` or `mcp-agent-mail serve http`

**Discovery**:
```bash
am --robot-help      # Full CLI docs
am --robot-status    # Health check
am tools             # List 45 MCP tools
```

**Core MCP Tools**:

| Category | Tools |
|----------|-------|
| Project | `ensure_project`, `list_projects`, `get_project_info` |
| Agent | `register_agent`, `list_agents`, `whois` |
| Messaging | `send_message`, `reply_message`, `check_inbox`, `list_outbox` |
| Threads | `list_threads`, `summarize_thread` |
| Files | `file_reservation_paths`, `release_reservation`, `list_file_reservations` |
| Build | `acquire_build_slot`, `release_build_slot` |

**Setup Workflow**:
1. `ensure_project` (slug=repo path, human_key=name)
2. `register_agent` (project_slug, name, program, model, task_description)
3. `file_reservation_paths` (paths, ttl_seconds, exclusive=true) — BEFORE editing
4. `send_message` for communication
5. `release_reservation` when done

**Pre-commit Guard**: `AGENT_MAIL_GUARD_MODE=enforce|warn`, `AGENT_MAIL_BYPASS=1` (emergency)

---

## 🔄 LAYER 2: SESSION WORKFLOW

### Git Worktrees (Sandbox Isolation)
Use worktrees for isolated work — no stashing needed.
```bash
git worktree add ../feature-x -b feature-x  # Create
git worktree remove ../feature-x            # Clean up
git worktree list                           # Show all
```

### Starting a Session
```bash
cm context "<task>"     # 1. Hydrate context
cm doctor              # 2. Health check
bd ready               # 3. Find work
bd show <id>           # 4. View details
bd update <id> --notes "Starting"  # 5. Claim
```

### During Work
```bash
cass search "problem" --robot --limit 5  # Search history
bd create "Found bug" --deps discovered-from:<id>  # Track discoveries
```

### Ending a Session (MANDATORY)
```bash
# 1. Create issues for remaining work
bd create "Follow-up" -t task -p 2

# 2. Quality gates (if code changed)
cargo check && cargo clippy -- -D warnings && cargo fmt --check

# 3. Update issues
bd close <id> --reason "Done"

# 4. PUSH (NON-NEGOTIABLE)
git pull --rebase && bd sync && git status  # Must show "up to date"

# 5. Provide follow-up prompt
```

---

## 🎯 LAYER 3: PROJECT-SPECIFIC

### Overview
**MCP Agent Mail** — Production-grade multi-agent messaging in Rust. 44.6x faster than Python (15,200 req/s).

### Repository Structure
```
crates/
├── libs/
│   ├── lib-core/     # Domain logic (BMC pattern)
│   ├── lib-mcp/      # 45 MCP tools
│   └── lib-server/   # HTTP layer (Axum 0.8)
└── services/
    ├── mcp-agent-mail/   # Unified CLI
    └── web-ui-leptos/    # Leptos frontend
```

### MANDATORY: Parallel Agent Workflow (ULTRA Pattern)

```
╔════════════════════════════════════════════════════════════════╗
║  🚨 AGENTS NEVER WORK ON MAIN BRANCH 🚨                        ║
║  ✅ Work ONLY on beads-sync or feature branches                ║
║  ✅ Use worktrees (.sandboxes/agent-<id>/)                     ║
║  ✅ Coordinator merges beads-sync → main                       ║
╚════════════════════════════════════════════════════════════════╝
```

**Three Layers**:
1. **File Reservations** (Agent Mail) — Logical locks, prevent same-file conflicts
2. **Worktrees** — Physical isolation, no stash/pop
3. **beads-sync** — Integration branch, bd sync commits here

**Agent Startup**:
```bash
git checkout beads-sync && git merge main --no-edit
# Register agent via MCP register_agent
# Reserve files via MCP file_reservation_paths (BEFORE worktree)
git worktree add .sandboxes/agent-$ID -b feature/<task> beads-sync
bd --no-daemon update <task> --status=in_progress
```

**Agent Completion**:
```bash
# Release reservations via MCP release_reservation
bd --no-daemon close <task>
cd ../.. && git checkout beads-sync
git merge feature/<task> --no-edit && git push origin beads-sync
git worktree remove .sandboxes/agent-$ID
```

### Quality Gates
```bash
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo test -p lib-core --test integration -- --test-threads=1
```

---

## 🔧 LAYER 4: LANGUAGE SPECIFIC

**Package Manager**: `cargo` only | **Edition**: 2024 | **Node.js**: `bun`

### Code Style

**BMC Pattern** (all business logic):
```rust
pub struct FooBmc;
impl FooBmc {
    pub async fn create(mm: &ModelManager, data: FooForCreate) -> Result<Foo> { ... }
}
```

**Error Handling**: Use `Result<T, E>` with `?` — never `unwrap()` in src/

**Strong Types**: Use newtypes (`ProjectSlug(String)`) not primitives

---

## 🤖 LAYER 5: MULTI-AGENT ORCHESTRATION

### Agent Roles
| Role | Name | Responsibility |
|------|------|----------------|
| Worker | `worker-<id>` | Implements, runs quality gates |
| Reviewer | `reviewer` | Validates, fixes issues |
| Human | `human` | Final oversight |

### Workflow
```
BEADS → WORKER → [COMPLETION] mail → REVIEWER → [APPROVED/FIXED] → HUMAN
        (exits)    (async)           (picks up)
```

**Key**: Worker sends [COMPLETION] and EXITS. Does NOT wait for [APPROVED].

### State Machine (Subject Prefixes)
```
[TASK_STARTED] → [COMPLETION] → [REVIEWING] → [APPROVED|REJECTED|FIXED] → [ACK]
     Worker        Worker         Reviewer         Reviewer             Human
```

### Worker Phase
1. `bd ready` → `bd update <id> --status in_progress`
2. `register_agent` → `file_reservation_paths`
3. Create worktree, implement, run quality gates
4. Commit, merge to beads-sync
5. Send `[COMPLETION]` mail (to=reviewer, cc=human, ack_required=true)
6. Release reservations, EXIT

### Reviewer Phase
1. `check_inbox` for `[COMPLETION]` mails
2. Check thread state (skip if `[APPROVED]` or `[REVIEWING]` exists)
3. Send `[REVIEWING]` to claim
4. **Validate**: Read files, check placeholders, verify acceptance criteria, run gates
5. If PASS: Send `[APPROVED]`, close task
6. If FAIL: Fix in worktree, send `[FIXED]`, close task

### Validation Checklist
- Zero `todo!()`, `unimplemented!()` in src/
- 100% acceptance criteria mapped to code
- All quality gates pass
- No OWASP vulnerabilities

### Message Templates

**[COMPLETION]** (Worker→Reviewer):
```markdown
## Task Completion Report
**Task ID**: <id> | **Commit**: <sha>
### Files Changed: <list>
### Acceptance Criteria: [x] done [x] done
### Quality Gates: ✅ check ✅ clippy ✅ fmt ✅ test
```

**[APPROVED]** (Reviewer→Worker, cc Human):
```markdown
## Review Complete - APPROVED
Implementation complete, criteria met, gates passed.
```

### Single-Agent Fallback
If no reviewer: Worker self-reviews and sends `[COMPLETION] (Self-Reviewed)` directly to Human.

### CC Rules
- Worker [COMPLETION] → CC: human
- Reviewer [REVIEWING/APPROVED/FIXED] → To: worker, CC: human
- Human [ACK] → To: reviewer, CC: worker

---

## 🆘 TROUBLESHOOTING

| Problem | Solution |
|---------|----------|
| `cass` launches TUI | Use `--robot` flag |
| `bd` shows "database not found" | `bd init --quiet` |
| bd in worktree fails | Use `bd --no-daemon` |
| Test DB conflicts | `--test-threads=1` |

---

**Version**: 2.0.0 (Compressed) | **Last Updated**: 2025-12-22

*Run `cm context "<task>"` and `bd ready` to get oriented.*
