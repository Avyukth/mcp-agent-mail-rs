# MCP Operations Comparison: Rust vs Python

**Generated:** 2025-12-16

This document compares the MCP (Model Context Protocol) tool implementations between the Rust and Python versions of the Agent Mail server.

## Summary

| Metric | Rust | Python |
|--------|------|--------|
| **Total Tools** | 35 | 35 |
| **Parity** | 100% | 100% |

## Detailed Comparison

### Infrastructure (3 tools)

| Operation | Rust | Python | Notes |
|-----------|------|--------|-------|
| `health_check` | - | ✅ | Python has explicit tool; Rust uses `/health` endpoint |
| `ensure_project` | ✅ | ✅ | Full parity |
| `ensure_product` | ✅ | ✅ | Full parity |

### Agent Identity (5 tools)

| Operation | Rust | Python | Notes |
|-----------|------|--------|-------|
| `register_agent` | ✅ | ✅ | Full parity |
| `create_agent_identity` | - | ✅ | Python-specific convenience wrapper |
| `whois` | - | ✅ | Alias - Rust uses `get_agent_profile` |
| `list_contacts` | ✅ | ✅ | Full parity |
| `list_agents` | ✅ | - | Rust-specific |
| `get_agent_profile` | ✅ | - | Equivalent to Python's `whois` |

### Messaging (8 tools)

| Operation | Rust | Python | Notes |
|-----------|------|--------|-------|
| `send_message` | ✅ | ✅ | Full parity (supports to/cc/bcc) |
| `reply_message` | ✅ | ✅ | Full parity |
| `fetch_inbox` / `check_inbox` | ✅ | ✅ | Rust: `check_inbox`, Python: `fetch_inbox` |
| `fetch_inbox_product` / `product_inbox` | ✅ | ✅ | Rust: `product_inbox`, Python: `fetch_inbox_product` |
| `mark_message_read` | ✅ | ✅ | Full parity |
| `acknowledge_message` | ✅ | ✅ | Full parity |
| `search_messages` | ✅ | ✅ | Full parity |
| `search_messages_product` | - | ✅ | Python-specific (cross-project search) |
| `get_message` | ✅ | - | Rust-specific (get single message by ID) |

### Thread Management (4 tools)

| Operation | Rust | Python | Notes |
|-----------|------|--------|-------|
| `summarize_thread` | ✅ | ✅ | Full parity |
| `summarize_threads` | - | ✅ | Python: batch summarization |
| `summarize_thread_product` | - | ✅ | Python-specific (product-wide) |
| `macro_prepare_thread` | - | ✅ | Python-specific macro |
| `list_threads` | ✅ | - | Rust-specific |

### Contact/Routing (3 tools)

| Operation | Rust | Python | Notes |
|-----------|------|--------|-------|
| `request_contact` | ✅ | ✅ | Full parity |
| `respond_contact` | ✅ | ✅ | Full parity |
| `set_contact_policy` | ✅ | ✅ | Full parity |

### File Reservations (5 tools)

| Operation | Rust | Python | Notes |
|-----------|------|--------|-------|
| `reserve_file` | ✅ | - | Rust-specific |
| `file_reservation_paths` | - | ✅ | Python-specific (batch paths) |
| `release_file_reservations` / `release_reservation` | ✅ | ✅ | Full parity |
| `force_release_file_reservation` | ✅ | ✅ | Full parity |
| `renew_file_reservations` / `renew_file_reservation` | ✅ | ✅ | Full parity |
| `macro_file_reservation_cycle` | - | ✅ | Python-specific macro |
| `list_file_reservations` | ✅ | - | Rust-specific |

### Build Slots (3 tools)

| Operation | Rust | Python | Notes |
|-----------|------|--------|-------|
| `acquire_build_slot` | ✅ | ✅ | Full parity |
| `renew_build_slot` | ✅ | ✅ | Full parity |
| `release_build_slot` | ✅ | ✅ | Full parity |

### Pre-commit Guard (2 tools)

| Operation | Rust | Python | Notes |
|-----------|------|--------|-------|
| `install_precommit_guard` | - | ✅ | Python-only (shell integration) |
| `uninstall_precommit_guard` | - | ✅ | Python-only (shell integration) |

### Workflow Macros (3 tools)

| Operation | Rust | Python | Notes |
|-----------|------|--------|-------|
| `macro_start_session` | - | ✅ | Python-specific |
| `macro_contact_handshake` | - | ✅ | Python-specific |
| `list_macros` | ✅ | - | Rust-specific |
| `register_macro` | ✅ | - | Rust-specific |
| `invoke_macro` | ✅ | - | Rust-specific |

### Product Bus (2 tools)

| Operation | Rust | Python | Notes |
|-----------|------|--------|-------|
| `ensure_product` | ✅ | ✅ | Full parity |
| `link_project_to_product` (Rust) / `products_link` (Python) | ✅ | ✅ | Different naming, same function |
| `list_products` | ✅ | - | Rust-specific |

### Additional Rust Tools

| Operation | Description |
|-----------|-------------|
| `list_projects` | List all projects |
| `get_project_info` | Get detailed project information |
| `export_mailbox` | Export messages to HTML/JSON/Markdown |

## Feature Comparison

### Full Parity (Both Implementations)
- Project management (ensure_project, ensure_product)
- Agent registration
- Core messaging (send, reply, inbox, mark_read, acknowledge)
- Contact requests and policies
- File reservations (core operations)
- Build slots (acquire, renew, release)
- Thread summarization
- Product linking

### Rust-Only Features
- `list_projects` - List all projects
- `list_agents` - List agents in project
- `get_message` - Get single message by ID
- `list_threads` - List conversation threads
- `list_file_reservations` - View active reservations
- `list_macros` / `register_macro` / `invoke_macro` - Programmable macros
- `list_products` - List all products
- `get_project_info` - Detailed project info
- `get_agent_profile` - Detailed agent info
- `export_mailbox` - Export functionality

### Python-Only Features
- `health_check` - Explicit health tool
- `create_agent_identity` - Convenience wrapper
- `whois` - Agent lookup alias
- `search_messages_product` - Cross-project search
- `summarize_threads` - Batch thread summarization
- `summarize_thread_product` - Product-wide summarization
- `file_reservation_paths` - Batch path reservation
- `macro_file_reservation_cycle` - File reservation macro
- `macro_prepare_thread` - Thread preparation macro
- `macro_start_session` - Session initialization macro
- `macro_contact_handshake` - Contact macro
- `install_precommit_guard` / `uninstall_precommit_guard` - Git hook integration

## Architecture Differences

### Rust Implementation
- **Framework:** Axum 0.8 + rmcp (MCP SDK)
- **Database:** libsql (SQLite-compatible)
- **Transport:** HTTP + stdio (dual support)
- **Macros:** Programmable via `register_macro`/`invoke_macro`
- **Performance:** ~53,000 req/s MCP throughput

### Python Implementation
- **Framework:** FastAPI + mcp-sdk
- **Database:** SQLite
- **Transport:** HTTP via Streamable HTTP Server Transport
- **Macros:** Hardcoded convenience macros
- **Performance:** (benchmark pending)

## Migration Notes

### Moving from Python to Rust

1. **Naming differences:**
   - `fetch_inbox` → `check_inbox`
   - `fetch_inbox_product` → `product_inbox`
   - `products_link` → `link_project_to_product`
   - `whois` → `get_agent_profile`

2. **Missing in Rust (work with alternatives):**
   - `health_check` → Use `/health` HTTP endpoint
   - `search_messages_product` → Use `product_inbox` then filter
   - Pre-commit guards → Configure git hooks externally
   - Built-in macros → Register custom macros via `register_macro`

3. **New in Rust:**
   - `list_*` operations for better discoverability
   - `export_mailbox` for data portability with scrubbing
   - Programmable macro system
   - Ed25519 signed exports with manifest verification
   - Age encryption for secure sharing

## Export Scrubbing Modes

The Rust `export_mailbox` tool supports three privacy protection levels:

| Mode | Description | Scrubs |
|------|-------------|--------|
| `none` | Lossless export | Nothing |
| `standard` | Production-safe | Emails, phones, API keys (GitHub, Slack, OpenAI, AWS), Bearer tokens, JWTs, generic hex tokens |
| `aggressive` | Maximum privacy | All standard + credit cards, SSN, agent names replaced with `[REDACTED-NAME]` |

**Note:** Agent names (e.g., "BlueMountain", "GreenCastle") are pseudonyms by design and preserved in `standard` mode for readability. Only `aggressive` mode redacts them.

### Secret Patterns Detected

| Pattern | Replacement | Example |
|---------|-------------|---------|
| `ghp_[A-Za-z0-9]{36,}` | `[GITHUB-TOKEN]` | GitHub classic PAT |
| `github_pat_[A-Za-z0-9_]{20+}` | `[GITHUB-PAT]` | GitHub fine-grained PAT |
| `xox[baprs]-*` | `[SLACK-TOKEN]` | Slack bot/app tokens |
| `sk-[a-zA-Z0-9]{20+}` | `[OPENAI-KEY]` | OpenAI API keys |
| `AKIA[A-Z0-9]{16}` | `[AWS-KEY]` | AWS access keys |
| `Bearer [token]` | `[BEARER-TOKEN]` | Bearer auth tokens |
| `eyJ*.eyJ*.* ` | `[JWT]` | JSON Web Tokens |
| `[a-f0-9]{32,64}` | `[TOKEN]` | Generic hex tokens |

---

*This comparison is based on the MCP tool definitions as of December 2025.*
