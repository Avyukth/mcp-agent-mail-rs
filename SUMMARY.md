# MCP Agent Mail (Rust Implementation)

## Project Goal
To re-implement the `mcp_agent_mail` system ("Gmail for coding agents") using a "Rust Native" approach. This involves replacing the original Python stack (FastAPI, SQLite+FTS5, GitPython) with high-performance, safe, and idiomatic Rust alternatives (Axum, Libsql, Git2) while adding a modern SvelteKit frontend.

The project follows strict quality protocols (A+ code standards, TDD, zero technical debt) derived from the `Depyler` project's guidelines.

## Achievements So Far (as of Phase 1 Completion)

1.  **Architecture & Setup**:
    *   Established a Rust workspace (`mcp-agent-mail-rs`) with modular crates:
        *   `lib-core`: Domain logic, data models, and storage abstraction.
        *   `mcp-server`: Axum-based web server (skeleton).
        *   `mcp-cli`: Command-line interface.
    *   Configured "Rust Native" dependencies: `libsql` (database), `git2` (git operations), `axum` (web), `tokio` (async runtime), `thiserror` (error handling).

2.  **Core Domain Logic (`lib-core`)**:
    *   Defined all data models (Agent, Message, Project, etc.) as Rust structs.
    *   Implemented the **Backend Model Controller (BMC)** pattern for data access.
    *   Created a robust `Error` type handling database, git, and migration errors.

3.  **Storage Engines**:
    *   **Database**: Successfully integrated `libsql` (compatible with Turso/SQLite). Implemented manual schema migrations for tables and FTS5 search.
    *   **Git**: Implemented a `git_store` module using `git2` to handle the "mailbox" storage pattern (committing message files).

4.  **CLI Implementation**:
    *   Built a functional CLI tool (`mcp-cli`) with commands:
        *   `create-project`: Initializes DB record and Git archive.
        *   `create-agent`: Registers an agent.
        *   `send-message`: Creates a message, saves to DB, writes to Git (canonical/inbox/outbox), and commits changes.
    *   Verified end-to-end flow: CLI commands run successfully and persist data.

## Planned Work (Upcoming)

### Phase 2: Frontend Scaffolding (SvelteKit)
*   Initialize `crates/services/web-ui` with SvelteKit + Bun.
*   Configure TailwindCSS with Material Design 3 theming.
*   Set up `adapter-static` for embedding in the Rust binary.

### Phase 3: Backend Web Integration
*   Update `mcp-server` to serve the static frontend assets.
*   Implement API endpoints in `mcp-server` mirroring the BMC logic to serve the frontend.
*   Configure CORS and proxying for seamless local development.

### Phase 4: Full MCP Protocol Support
*   Integrate `mcp-protocol-sdk` to implement the actual Model Context Protocol tools (`read_inbox`, etc.).
*   Connect MCP tools to the `lib-core` business logic.

### Phase 5: Search & Polish
*   Implement search functionality using Libsql's FTS5 features.
*   Comprehensive E2E testing and final quality audit (15-tool validation).

## Current Status
**Phase 1 Complete.** The Rust backend core is functional and verified via CLI. We are now ready to start the Frontend Phase.
