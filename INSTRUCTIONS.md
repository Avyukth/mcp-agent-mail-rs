# Agent Mail Instructions for AI Agents

> MCP Agent Mail coordination protocol for multi-agent systems.
> Tool names are specific to `mcp-agent-mail-rs` (the `am` CLI).

---

## Quick Start Sequence

```
1. ensure_project       → Create/verify project exists
2. register_agent       → Get your identity (auto-generated name)
3. check_inbox          → See if there are instructions waiting
4. file_reservation_paths → Reserve files before editing
   ... do work ...
5. send_message         → Report progress/completion
6. release_reservation  → Release file locks
```

---

## Step 1: Register Identity

### Create/Verify Project

```json
{
  "tool": "ensure_project",
  "args": {
    "slug": "my-project",
    "human_key": "/absolute/path/to/repo"
  }
}
```

**Returns:**
```json
{
  "slug": "my-project",
  "human_key": "/absolute/path/to/repo",
  "created_at": "2025-12-22T10:00:00Z"
}
```

### Register as Agent

```json
{
  "tool": "register_agent",
  "args": {
    "project_slug": "my-project",
    "program": "claude-code",
    "model": "opus",
    "task_description": "Implementing authentication feature"
  }
}
```

> **Note:** Omit `name` field to let server auto-generate an "adjective-noun" identity (e.g., "bright-falcon").

**Returns:**
```json
{
  "name": "bright-falcon",
  "program": "claude-code",
  "model": "opus",
  "inception_ts": "2025-12-22T10:00:00Z"
}
```

---

## Step 2: Reserve Files Before Editing

### Reserve Multiple Paths (Glob Patterns)

```json
{
  "tool": "file_reservation_paths",
  "args": {
    "project_slug": "my-project",
    "agent_name": "bright-falcon",
    "paths": ["src/**/*.rs", "Cargo.toml"],
    "ttl_seconds": 3600,
    "exclusive": true,
    "reason": "Implementing auth module"
  }
}
```

### Reserve Single File

```json
{
  "tool": "reserve_file",
  "args": {
    "project_slug": "my-project",
    "agent_name": "bright-falcon",
    "path": "src/auth.rs",
    "ttl_seconds": 3600,
    "exclusive": true
  }
}
```

### List Active Reservations

```json
{
  "tool": "list_file_reservations",
  "args": {
    "project_slug": "my-project"
  }
}
```

### Renew Reservation (Extend TTL)

```json
{
  "tool": "renew_file_reservation",
  "args": {
    "reservation_id": "uuid-here",
    "ttl_seconds": 3600
  }
}
```

### Release Reservation

```json
{
  "tool": "release_reservation",
  "args": {
    "reservation_id": "uuid-here"
  }
}
```

### Force Release (Emergency Override)

```json
{
  "tool": "force_release_reservation",
  "args": {
    "reservation_id": "uuid-here",
    "reason": "Agent crashed, manual cleanup"
  }
}
```

---

## Step 3: Check Inbox

```json
{
  "tool": "check_inbox",
  "args": {
    "project_slug": "my-project",
    "agent_name": "bright-falcon",
    "limit": 20
  }
}
```

**Returns:**
```json
{
  "messages": [
    {
      "id": 123,
      "from": "coordinator",
      "subject": "Task Assignment",
      "body_md": "Please implement...",
      "importance": "high",
      "thread_id": "FEAT-123",
      "sent_at": "2025-12-22T09:00:00Z",
      "read": false,
      "acknowledged": false
    }
  ]
}
```

---

## Step 4: Acknowledge Messages

### Mark as Read Only

```json
{
  "tool": "mark_message_read",
  "args": {
    "message_id": 123
  }
}
```

### Full Acknowledgment (Read + Ack)

```json
{
  "tool": "acknowledge_message",
  "args": {
    "message_id": 123
  }
}
```

---

## Step 5: Send Messages

### Send New Message

```json
{
  "tool": "send_message",
  "args": {
    "project_slug": "my-project",
    "sender_name": "bright-falcon",
    "to": "coordinator,other-agent",
    "cc": "",
    "bcc": "",
    "subject": "Completed: Authentication Module",
    "body_md": "## Summary\n\nImplemented JWT-based auth...\n\n## Files Changed\n- src/auth.rs\n- src/middleware.rs",
    "importance": "normal",
    "thread_id": "FEAT-123"
  }
}
```

**Importance levels:** `normal`, `high`, `urgent`

### Reply to Message

```json
{
  "tool": "reply_message",
  "args": {
    "project_slug": "my-project",
    "sender_name": "bright-falcon",
    "message_id": 123,
    "body_md": "Acknowledged. Starting work now."
  }
}
```

### List Sent Messages (Outbox)

```json
{
  "tool": "list_outbox",
  "args": {
    "project_slug": "my-project",
    "agent_name": "bright-falcon"
  }
}
```

---

## Step 6: Search & Discovery

### Get Specific Message

```json
{
  "tool": "get_message",
  "args": {
    "message_id": 123
  }
}
```

### Search Messages

```json
{
  "tool": "search_messages",
  "args": {
    "project_slug": "my-project",
    "query": "authentication error",
    "limit": 20
  }
}
```

### List Threads

```json
{
  "tool": "list_threads",
  "args": {
    "project_slug": "my-project"
  }
}
```

### Summarize Thread

```json
{
  "tool": "summarize_thread",
  "args": {
    "project_slug": "my-project",
    "thread_id": "FEAT-123"
  }
}
```

---

## Step 7: Agent Discovery

### List All Agents in Project

```json
{
  "tool": "list_agents",
  "args": {
    "project_slug": "my-project"
  }
}
```

### Get Agent Profile

```json
{
  "tool": "get_agent_profile",
  "args": {
    "project_slug": "my-project",
    "agent_name": "bright-falcon"
  }
}
```

### List All Projects

```json
{
  "tool": "list_projects",
  "args": {}
}
```

### Get Project Info

```json
{
  "tool": "get_project_info",
  "args": {
    "project_slug": "my-project"
  }
}
```

---

## Step 8: Contact Management

### Request Contact (Cross-Agent Permission)

```json
{
  "tool": "request_contact",
  "args": {
    "project_slug": "my-project",
    "requester_name": "bright-falcon",
    "target_name": "swift-eagle"
  }
}
```

### Respond to Contact Request

```json
{
  "tool": "respond_contact",
  "args": {
    "request_id": "uuid-here",
    "accept": true
  }
}
```

### List Contacts

```json
{
  "tool": "list_contacts",
  "args": {
    "project_slug": "my-project",
    "agent_name": "bright-falcon"
  }
}
```

### Set Contact Policy

```json
{
  "tool": "set_contact_policy",
  "args": {
    "project_slug": "my-project",
    "agent_name": "bright-falcon",
    "policy": "contacts_only"
  }
}
```

**Policies:** `open`, `auto`, `contacts_only`, `block_all`

---

## Step 9: Build Slot Coordination (CI/CD)

### Acquire Build Slot

```json
{
  "tool": "acquire_build_slot",
  "args": {
    "project_slug": "my-project",
    "agent_name": "bright-falcon",
    "ttl_seconds": 600
  }
}
```

### Renew Build Slot

```json
{
  "tool": "renew_build_slot",
  "args": {
    "slot_id": "uuid-here",
    "ttl_seconds": 600
  }
}
```

### Release Build Slot

```json
{
  "tool": "release_build_slot",
  "args": {
    "slot_id": "uuid-here"
  }
}
```

---

## Step 10: Product Coordination (Multi-Repo)

### Create/Ensure Product

```json
{
  "tool": "ensure_product",
  "args": {
    "uid": "my-product",
    "name": "My Product Suite"
  }
}
```

### Link Project to Product

```json
{
  "tool": "link_project_to_product",
  "args": {
    "product_uid": "my-product",
    "project_slug": "my-project"
  }
}
```

### List Products

```json
{
  "tool": "list_products",
  "args": {}
}
```

### Product-Wide Inbox

```json
{
  "tool": "product_inbox",
  "args": {
    "product_uid": "my-product",
    "agent_name": "bright-falcon"
  }
}
```

### Search Across Product

```json
{
  "tool": "search_messages_product",
  "args": {
    "product_uid": "my-product",
    "query": "release blocker"
  }
}
```

### Summarize Thread Across Product

```json
{
  "tool": "summarize_thread_product",
  "args": {
    "product_uid": "my-product",
    "thread_id": "RELEASE-1.0"
  }
}
```

---

## Step 11: Attachments

### Add Attachment to Message

```json
{
  "tool": "add_attachment",
  "args": {
    "message_id": 123,
    "filename": "screenshot.png",
    "content_base64": "iVBORw0KGgo..."
  }
}
```

### Get Attachment

```json
{
  "tool": "get_attachment",
  "args": {
    "attachment_id": "uuid-here"
  }
}
```

---

## Step 12: Observability

### List Tool Metrics

```json
{
  "tool": "list_tool_metrics",
  "args": {
    "project_slug": "my-project",
    "limit": 50
  }
}
```

### Get Tool Stats

```json
{
  "tool": "get_tool_stats",
  "args": {
    "project_slug": "my-project"
  }
}
```

### List Activity

```json
{
  "tool": "list_activity",
  "args": {
    "project_slug": "my-project",
    "limit": 50
  }
}
```

### List Pending Reviews

```json
{
  "tool": "list_pending_reviews",
  "args": {
    "project_slug": "my-project"
  }
}
```

---

## Step 13: Pre-Commit Guard

### Install Guard

```json
{
  "tool": "install_precommit_guard",
  "args": {
    "project_slug": "my-project"
  }
}
```

### Uninstall Guard

```json
{
  "tool": "uninstall_precommit_guard",
  "args": {
    "project_slug": "my-project"
  }
}
```

---

## Step 14: Export & Archive

### Export Mailbox

```json
{
  "tool": "export_mailbox",
  "args": {
    "project_slug": "my-project",
    "format": "markdown"
  }
}
```

**Formats:** `html`, `json`, `markdown`

---

## Tool Name Mapping (vs Other Docs)

| Other Docs Say | This System Uses |
|----------------|------------------|
| `fetch_inbox` | `check_inbox` |
| `whois` | `get_agent_profile` |
| `release_file_reservations` | `release_reservation` |
| `renew_file_reservations` | `renew_file_reservation` |
| `list_reservations` | `list_file_reservations` |
| `create_agent_identity` | `register_agent` (omit name) |
| `macro_start_session` | Manual: ensure_project + register_agent + check_inbox |

---

## Complete Tool List (47 Tools)

### Project & Agent Management
- `ensure_project` - Create/verify project
- `register_agent` - Register agent identity
- `list_projects` - List all projects
- `list_agents` - List agents in project
- `get_project_info` - Project details
- `get_agent_profile` - Agent details

### Messaging
- `send_message` - Send to agents
- `reply_message` - Reply to message
- `check_inbox` - Get inbox messages
- `get_message` - Get specific message
- `mark_message_read` - Mark as read
- `acknowledge_message` - Full acknowledgment
- `list_outbox` - Sent messages
- `search_messages` - Search messages
- `list_threads` - List conversation threads
- `summarize_thread` - Summarize thread

### File Reservations
- `reserve_file` - Reserve single file
- `file_reservation_paths` - Reserve multiple paths
- `list_file_reservations` - List reservations
- `renew_file_reservation` - Extend TTL
- `release_reservation` - Release lock
- `force_release_reservation` - Emergency release

### Contacts
- `request_contact` - Request contact permission
- `respond_contact` - Accept/reject request
- `list_contacts` - List contacts
- `set_contact_policy` - Set contact policy

### Build Slots
- `acquire_build_slot` - Get exclusive CI slot
- `release_build_slot` - Release slot
- `renew_build_slot` - Extend slot TTL

### Macros
- `list_macros` - List available macros
- `register_macro` - Register new macro
- `invoke_macro` - Execute macro

### Products (Multi-Repo)
- `ensure_product` - Create/get product
- `link_project_to_product` - Link project
- `list_products` - List products
- `product_inbox` - Product-wide inbox
- `search_messages_product` - Search across product
- `summarize_thread_product` - Summarize across product

### Attachments
- `add_attachment` - Add to message
- `get_attachment` - Get attachment

### Observability
- `list_tool_metrics` - Tool usage metrics
- `get_tool_stats` - Aggregated stats
- `list_activity` - Recent activity
- `list_pending_reviews` - Pending acks

### Guards
- `install_precommit_guard` - Install hook
- `uninstall_precommit_guard` - Remove hook

### Export
- `export_mailbox` - Export to HTML/JSON/MD

---

## Error Handling

Common errors and resolutions:

| Error | Cause | Resolution |
|-------|-------|------------|
| `agent not registered` | Called tool before register_agent | Run ensure_project + register_agent first |
| `FILE_RESERVATION_CONFLICT` | Another agent holds the file | Wait for expiry or use non-exclusive |
| `message not found` | Invalid message_id | Verify ID from check_inbox |
| `project not found` | Invalid project_slug | Run ensure_project first |
| `unauthorized` | Missing/invalid auth | Check bearer token |

---

## Best Practices

1. **Always register first** - Call ensure_project + register_agent before any other operations
2. **Reserve before editing** - Get file reservations before modifying code
3. **Use thread_id** - Group related messages with consistent thread_id
4. **Release promptly** - Release reservations when done, don't let them expire
5. **Acknowledge important messages** - Use acknowledge_message for high/urgent messages
6. **Check inbox regularly** - Poll check_inbox for new instructions
7. **Use products for multi-repo** - Link related projects to a product for unified coordination
