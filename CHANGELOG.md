# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Testing Infrastructure**
  - Archive and attachments tool tests (archive_tests.rs, attachments_tests.rs)
  - Comprehensive tests for macros, export, reviews, files, precommit modules
  - TDD tests for macro tools (test_macro_tools.rs)
  - Schema auto-generation tests for MCP tools
  - Cargo-mutants configuration for mutation testing

- **Web UI Enhancements**
  - GitHub Pages static deployment support with git-based workflow
  - Factory.ai design system adoption with spotlight effects
  - Bulk selection and actions for projects
  - Sorting controls for project and agent list views
  - Content-aware loading skeleton screens
  - Reusable EmptyState component with consistent CTAs
  - Delete UI for agents with dropdown menu and confirmation
  - Export button in bulk action bar

- **API & Server**
  - Readiness probe with database connectivity check (`/api/ready`)
  - Request body size limits for DoS protection
  - NTM compatibility alias schemas for MCP tools

- **Code Quality**
  - PMAT-recommended lints enabled workspace-wide
  - Comprehensive rustdoc documentation for public APIs
  - 4 convenience workflow tools (list_builtin_workflows, quick_standup_workflow, quick_handoff_workflow, quick_review_workflow)
  - Pre-commit guard MCP tools (install_precommit_guard, uninstall_precommit_guard)
  - Rust-native pre-commit hooks via cargo-husky

- **Documentation**
  - GitHub Pages deployment environment variables in .env.example
  - Improved rustdoc coverage to 90% for lib-core

### Changed
- **Type Safety**: Migrated to ProjectId/AgentId newtypes across entire codebase
- Theme toggle button now uses inline SVG instead of Lucide icons for better reliability
- Seamless sticky toolbar implementation across all viewports
- Button and project card layout improvements
- Restored 8px rounded corners for softer aesthetic

### Fixed
- Disabled autobenches to prevent duplicate target warning
- MessageForCreate API usage in attachments tests (correct Option types)
- Reduced commit_message_to_git complexity from 12 to 6
- Eliminated panic risk in static_files.rs
- Theme button functionality after embedding
- Import ordering in test modules
- Dropdown menu closes before opening delete confirmation dialog
- .nojekyll added to prevent Jekyll from ignoring _app folder
- Svelte 5 runes compatibility (mode.current usage)
- Newtype ID conversions across test files

## [0.1.0] - 2025-12-17

### Added
- **Core Infrastructure**
  - Multi-agent messaging system with async coordination
  - SQLite database with libsql for persistence
  - Git archive integration for message storage
  - 45 MCP tools for agent-to-agent communication

- **MCP Protocol Support**
  - STDIO transport for Claude Desktop integration
  - SSE transport for web-based clients
  - JSON-RPC 2.0 protocol implementation
  - Full MCP specification compliance

- **Agent Management**
  - Agent registration and identity management
  - Agent profiles and capabilities tracking
  - Cross-project agent contacts
  - Contact request/response workflow

- **Messaging System**
  - Send/receive messages with multiple recipients
  - Thread-based conversation tracking
  - Read status and acknowledgments
  - Message search across projects
  - Inbox/outbox management

- **File Coordination**
  - File reservation system (exclusive/shared locks)
  - Path pattern matching with glob support
  - TTL-based automatic expiration
  - Force release for emergency override
  - Reservation renewal

- **Build Coordination**
  - Build slot management for CI/CD isolation
  - Concurrent build prevention
  - Slot renewal and release

- **Workflow Automation**
  - Macro/workflow registration and invocation
  - Built-in workflows (start_session, prepare_thread, file_reservation_cycle, contact_handshake, broadcast_message)
  - Custom macro definitions with JSON steps
  - Macro listing and management

- **Product Management**
  - Multi-repo coordination via products
  - Project-to-product linking
  - Product-wide inbox aggregation

- **API Endpoints**
  - RESTful HTTP API on port 8765
  - Health and readiness checks
  - JWT and Bearer token authentication
  - Rate limiting (100 req/min per token)
  - Attachment upload/download

- **Web UI** (Leptos WASM)
  - Inbox viewer with agent filtering
  - Message detail view
  - Compose message interface
  - Dark mode support
  - Responsive design

- **Development Tools**
  - Unified CLI (mcp-agent-mail binary)
  - Multiple server modes (http, mcp, stdio)
  - Database migrations (auto-run on start)
  - Environment-based configuration (12-factor)

- **Performance**
  - 44.6x throughput vs Python reference (15,200 vs 341 req/s)
  - Sub-10ms P99 latency for MCP calls
  - 62,316 req/s for health endpoint
  - Supports 100+ concurrent agents

- **Testing**
  - Integration tests for all BMC layers
  - MCP protocol compliance tests
  - Concurrent agent benchmarks

- **Documentation**
  - Architecture documentation (ARCHITECTURE.md)
  - Walkthrough guide (WALKTHROUGH.md)
  - Universal agent operating manual (AGENTS.md)
  - Integration configs for Claude, Cline, Cursor
  - MCP tool reference (45 tools)

### Security
- SQL injection prevention via parameterized queries
- Bearer token and JWT authentication
- Rate limiting to prevent abuse
- File path validation for attachments
- Git integration with safe operations

### Performance Metrics
- MCP Throughput: 15,200 req/s
- MCP P99 Latency: 7.2ms
- REST Health Endpoint: 62,316 req/s
- Concurrent Agent Support: 100+ verified

### Dependencies
- Rust 2024 edition
- Axum 0.8 (HTTP framework)
- libsql (SQLite driver)
- rmcp (MCP protocol)
- Leptos (WASM frontend)
- tokio (async runtime)

### Known Issues
- None

---

## Release Links
- [v0.1.0](https://github.com/Avyukth/mcp-agent-mail-rs/releases/tag/v0.1.0) - Initial release

## Repository
- GitHub: https://github.com/Avyukth/mcp-agent-mail-rs
- Issues: https://github.com/Avyukth/mcp-agent-mail-rs/issues
