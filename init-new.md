# Autonomous Agent Initialization Guide

> **Purpose**: Enable AI agents to work autonomously on beads until completion, coordinating via MCP Agent Mail.

## Quick Start

```bash
# 1. Ensure server is running
am serve http --port 8765 &

# 2. Initialize agent identity
am tools  # List all 47 MCP tools available
```

---

## Phase 1: Agent Registration

### Step 1.1: Ensure Project Exists

```json
{
  "tool": "ensure_project",
  "arguments": {
    "slug": "/abs/path/to/mcp-agent-mail-rs",
    "human_key": "mcp-agent-mail-rs"
  }
}
```

### Step 1.2: Register Agent Identity

```json
{
  "tool": "register_agent",
  "arguments": {
    "project_slug": "mcp-agent-mail-rs",
    "name": "BlueMountain",
    "program": "claude-code",
    "model": "claude-opus-4",
    "task_description": "Working on NTM-001: Add tool aliases for NTM compatibility"
  }
}
```

**Naming Convention**: Use adjective+noun format (GreenCastle, RedFalcon, BlueMountain).

---

## Phase 2: Claim Work from Beads

### Step 2.1: Find Ready Work

```bash
bd ready --json
```

### Step 2.2: Claim and Communicate

```json
{
  "tool": "send_message",
  "arguments": {
    "project_slug": "mcp-agent-mail-rs",
    "sender_name": "BlueMountain",
    "to": "Coordinator",
    "subject": "[CLAIMING] NTM-001: Add tool aliases",
    "body_md": "I am claiming issue `mcp-agent-mail-rs-46xk` (NTM-001).\n\n**Plan:**\n1. Add alias mapping in call_tool\n2. Sync schema with implementations\n3. Add unit tests\n\n**Files to reserve:** `crates/libs/lib-mcp/src/tools/mod.rs`",
    "thread_id": "NTM-001",
    "importance": "normal"
  }
}
```

### Step 2.3: Reserve Files (BEFORE editing)

```json
{
  "tool": "file_reservation_paths",
  "arguments": {
    "project_slug": "mcp-agent-mail-rs",
    "agent_name": "BlueMountain",
    "paths": [
      "crates/libs/lib-mcp/src/tools/mod.rs",
      "crates/libs/lib-mcp/src/tools/*.rs"
    ],
    "ttl_seconds": 3600,
    "exclusive": true,
    "reason": "NTM-001: Adding tool aliases"
  }
}
```

**If reservation fails**: Another agent has the files. Check inbox and coordinate.

---

## Phase 3: Autonomous Work Loop

### The Autonomous Agent Loop

```
┌──────────────────────────────────────────────────────────────┐
│                    AUTONOMOUS WORK LOOP                       │
│                                                              │
│  ┌─────────────┐                                             │
│  │ check_inbox │◄───────────────────────────────────┐        │
│  └──────┬──────┘                                    │        │
│         │ New messages?                              │        │
│         ▼                                            │        │
│  ┌──────────────┐    Yes    ┌──────────────────┐    │        │
│  │ Process msg  │◄─────────│ Requires action? │    │        │
│  └──────┬───────┘           └────────┬─────────┘    │        │
│         │                             │ No          │        │
│         ▼                             ▼             │        │
│  ┌──────────────┐           ┌─────────────────┐    │        │
│  │   Do Work    │           │ mark_message_   │    │        │
│  │ (edit code)  │           │     read        │    │        │
│  └──────┬───────┘           └────────┬────────┘    │        │
│         │                            │             │        │
│         ▼                            │             │        │
│  ┌──────────────┐                    │             │        │
│  │ send_message │ ───────────────────┘             │        │
│  │ (progress)   │                                  │        │
│  └──────┬───────┘                                  │        │
│         │                                          │        │
│         ▼                                          │        │
│  ┌──────────────┐    No     ┌───────────────┐      │        │
│  │ Task done?   │──────────►│ renew_file_   │──────┘        │
│  └──────┬───────┘           │ reservation   │               │
│         │ Yes               └───────────────┘               │
│         ▼                                                   │
│  ┌──────────────┐                                           │
│  │ bd close     │                                           │
│  └──────┬───────┘                                           │
│         │                                                   │
│         ▼                                                   │
│  ┌────────────────────┐                                     │
│  │ release_reservation│                                     │
│  └────────────────────┘                                     │
└──────────────────────────────────────────────────────────────┘
```

### Step 3.1: Check Inbox Regularly

```json
{
  "tool": "check_inbox",
  "arguments": {
    "project_slug": "mcp-agent-mail-rs",
    "agent_name": "BlueMountain",
    "limit": 10
  }
}
```

### Step 3.2: Process Messages

**For blocking messages (ack_required=true)**:
```json
{
  "tool": "acknowledge_message",
  "arguments": {
    "project_slug": "mcp-agent-mail-rs",
    "message_id": 123,
    "agent_name": "BlueMountain"
  }
}
```

**For info-only messages**:
```json
{
  "tool": "mark_message_read",
  "arguments": {
    "project_slug": "mcp-agent-mail-rs",
    "message_id": 123,
    "agent_name": "BlueMountain"
  }
}
```

### Step 3.3: Report Progress

```json
{
  "tool": "send_message",
  "arguments": {
    "project_slug": "mcp-agent-mail-rs",
    "sender_name": "BlueMountain",
    "to": "Coordinator",
    "subject": "[PROGRESS] NTM-001: 50% complete",
    "body_md": "## Progress Update\n\n- [x] Added alias mapping in call_tool\n- [ ] Sync schema\n- [ ] Add tests\n\n**Blockers**: None\n**ETA**: Next commit",
    "thread_id": "NTM-001"
  }
}
```

### Step 3.4: Renew Reservations (if work takes longer)

```json
{
  "tool": "renew_file_reservation",
  "arguments": {
    "project_slug": "mcp-agent-mail-rs",
    "reservation_id": 42,
    "ttl_seconds": 3600
  }
}
```

### Step 3.5: Request Help from Other Agents

```json
{
  "tool": "send_message",
  "arguments": {
    "project_slug": "mcp-agent-mail-rs",
    "sender_name": "BlueMountain",
    "to": "GreenCastle,RedFalcon",
    "subject": "[HELP] Need review on NTM-001",
    "body_md": "Can someone review my changes to `call_tool` alias mapping?\n\n```rust\nlet tool_name = match request.name.as_str() {\n    \"fetch_inbox\" => \"list_inbox\",\n    other => other,\n}.to_string();\n```",
    "thread_id": "NTM-001",
    "ack_required": true
  }
}
```

---

## Phase 4: Coordination Patterns

### Pattern 1: Handoff to Another Agent

When blocked or need specialized help:

```json
{
  "tool": "send_message",
  "arguments": {
    "project_slug": "mcp-agent-mail-rs",
    "sender_name": "BlueMountain",
    "to": "GreenCastle",
    "subject": "[HANDOFF] NTM-003 requires lib-core changes",
    "body_md": "## Handoff Request\n\n**Task**: NTM-003 (create_agent_identity)\n**Reason**: Requires changes to lib-core name generation\n**Files needed**: `crates/libs/lib-core/src/model/agent.rs`\n\nI will release my reservation on lib-mcp files. Please claim lib-core.",
    "thread_id": "NTM-003",
    "importance": "high",
    "ack_required": true
  }
}
```

### Pattern 2: Parallel Work Notification

When discovering work that can be parallelized:

```json
{
  "tool": "send_message",
  "arguments": {
    "project_slug": "mcp-agent-mail-rs",
    "sender_name": "BlueMountain",
    "to": "Coordinator",
    "subject": "[PARALLEL] Found independent subtasks",
    "body_md": "## Parallel Work Available\n\nWhile working on NTM-001, I identified these can run in parallel:\n\n1. **NTM-002** (list_project_agents) - no file conflicts with NTM-001\n2. **NTM-004** (macro_start_session) - separate file set\n\nSuggestion: Spawn additional agents for throughput.",
    "thread_id": "NTM-COORDINATION"
  }
}
```

### Pattern 3: Conflict Resolution

When file reservation conflicts occur:

```json
{
  "tool": "send_message",
  "arguments": {
    "project_slug": "mcp-agent-mail-rs",
    "sender_name": "BlueMountain",
    "to": "RedFalcon",
    "subject": "[CONFLICT] File reservation overlap",
    "body_md": "## Conflict Detected\n\n**File**: `crates/libs/lib-mcp/src/tools/mod.rs`\n**My task**: NTM-001 (aliases)\n**Your task**: ?\n\n**Proposal**:\n1. I take lines 1700-1800 (call_tool function)\n2. You take other sections\n\nOr: I finish first (~30 min), then release.",
    "importance": "urgent",
    "ack_required": true
  }
}
```

---

## Phase 5: Task Completion

### Step 5.1: Close Bead

```bash
bd close mcp-agent-mail-rs-46xk --reason "Implemented alias mapping, schema sync, and tests"
```

### Step 5.2: Release Reservations

```json
{
  "tool": "release_reservation",
  "arguments": {
    "project_slug": "mcp-agent-mail-rs",
    "reservation_id": 42
  }
}
```

### Step 5.3: Send Completion Notification

```json
{
  "tool": "send_message",
  "arguments": {
    "project_slug": "mcp-agent-mail-rs",
    "sender_name": "BlueMountain",
    "to": "Coordinator",
    "subject": "[COMPLETED] NTM-001: Tool aliases",
    "body_md": "## Task Completed\n\n**Bead**: mcp-agent-mail-rs-46xk\n**Commits**: abc1234, def5678\n\n**Changes**:\n- Added alias mapping in `call_tool`\n- Synced schema: added `whois`, `fetch_inbox`\n- Added 4 unit tests\n\n**Ready for**: NTM-002, NTM-003, NTM-004 (unblocked)",
    "thread_id": "NTM-001",
    "importance": "high"
  }
}
```

### Step 5.4: Check for More Work

```bash
bd ready --json
```

If more work available, return to Phase 2.

---

## MCP Agent Mail Tools Reference (47 total)

### Core Messaging
| Tool | Description |
|------|-------------|
| `send_message` | Send message to agents (to, cc, bcc) |
| `reply_message` | Reply to existing thread |
| `check_inbox` | Get unread messages |
| `list_outbox` | Sent messages |
| `get_message` | Get message by ID |
| `mark_message_read` | Mark as read |
| `acknowledge_message` | Mark as acknowledged |

### Search & Threads
| Tool | Description |
|------|-------------|
| `search_messages` | Full-text search in project |
| `search_messages_product` | Search across product |
| `list_threads` | List conversation threads |
| `summarize_thread` | AI summary of thread |
| `summarize_thread_product` | Summary across product |
| `list_pending_reviews` | Messages awaiting ack |

### File Reservations
| Tool | Description |
|------|-------------|
| `file_reservation_paths` | Reserve multiple paths |
| `reserve_file` | Reserve single file |
| `release_reservation` | Release by ID |
| `list_file_reservations` | List active reservations |
| `renew_file_reservation` | Extend TTL |
| `force_release_reservation` | Emergency override |

### Agent Identity
| Tool | Description |
|------|-------------|
| `register_agent` | Register/update agent |
| `list_agents` | List project agents |
| `get_agent_profile` | Detailed agent info |
| `whois` | Agent lookup (alias) |

### Project Management
| Tool | Description |
|------|-------------|
| `ensure_project` | Create/get project |
| `list_projects` | All projects |
| `get_project_info` | Project details |

### Contacts & Policies
| Tool | Description |
|------|-------------|
| `request_contact` | Request contact permission |
| `respond_contact` | Accept/reject contact |
| `list_contacts` | Agent's contacts |
| `set_contact_policy` | open/auto/contacts_only/block_all |

### Build Slots (CI/CD)
| Tool | Description |
|------|-------------|
| `acquire_build_slot` | Exclusive build access |
| `release_build_slot` | Release slot |
| `renew_build_slot` | Extend slot TTL |

### Macros
| Tool | Description |
|------|-------------|
| `list_macros` | Available macros |
| `register_macro` | Define new macro |
| `invoke_macro` | Execute macro |

### Products (Multi-Repo)
| Tool | Description |
|------|-------------|
| `ensure_product` | Create product |
| `link_project_to_product` | Link project |
| `list_products` | All products |
| `product_inbox` | Aggregated inbox |

### Utilities
| Tool | Description |
|------|-------------|
| `export_mailbox` | Export to HTML/JSON/MD |
| `add_attachment` | Attach file to message |
| `get_attachment` | Retrieve attachment |
| `install_precommit_guard` | Install git hook |
| `uninstall_precommit_guard` | Remove git hook |
| `list_tool_metrics` | Usage metrics |
| `get_tool_stats` | Aggregated stats |
| `list_activity` | Project activity log |

---

## Beads Integration

### Essential Commands

```bash
# Find unblocked work
bd ready --json

# Claim work
bd update <id> --status in_progress

# Create discovered issues
bd create "Found bug" --description="Details" -t bug --deps discovered-from:<id>

# Complete work
bd close <id> --reason "Completed"

# Sync at session end
bd sync
```

### Status Flow

```
open → in_progress → [completed | blocked]
```

### Priority Levels

| Priority | Meaning |
|----------|---------|
| P0 | Critical (security, data loss) |
| P1 | High (major features, important bugs) |
| P2 | Medium (nice-to-have) |
| P3 | Low (polish) |
| P4 | Backlog |

---

## Autonomous Operation Rules

### MUST DO
- [x] Register identity before any work
- [x] Reserve files before editing
- [x] Check inbox regularly (every major step)
- [x] Acknowledge blocking messages
- [x] Report progress via send_message
- [x] Release reservations when done
- [x] Close beads when complete
- [x] Run `bd sync` at session end

### MUST NOT
- [ ] Edit files without reservation
- [ ] Ignore messages with `ack_required=true`
- [ ] Work on main branch directly
- [ ] Force push to shared branches
- [ ] Skip the completion notification

### SHOULD DO
- Renew reservations before expiry
- Notify on blockers immediately
- Request handoff when stuck
- Create follow-on beads for discovered work
- Summarize long threads for context

---

## Emergency Procedures

### Force Release (stuck reservation)
```json
{
  "tool": "force_release_reservation",
  "arguments": {
    "project_slug": "mcp-agent-mail-rs",
    "reservation_id": 42,
    "reason": "Agent unresponsive for >1 hour"
  }
}
```

### Urgent Message
```json
{
  "tool": "send_message",
  "arguments": {
    "sender_name": "BlueMountain",
    "to": "Coordinator",
    "subject": "[URGENT] Build broken",
    "importance": "urgent",
    "ack_required": true
  }
}
```

---

## Session End Checklist

```bash
[ ] All active beads closed or updated with notes
[ ] All file reservations released
[ ] Completion messages sent for finished work
[ ] bd sync completed
[ ] git status shows clean working tree
```

**Next agent prompt**:
```
Continue work on mcp-agent-mail-rs. Run `bd ready --json` to find unblocked tasks.
Check inbox for any pending messages: check_inbox(project_slug="mcp-agent-mail-rs", agent_name="<your-name>")
```
