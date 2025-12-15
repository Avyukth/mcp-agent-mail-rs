# Beads Environment Variables Reference

This document describes all environment variables used by Beads (`bd`), the issue tracking system used in this project.

## Table of Contents

- [Environment Variables Reference](#environment-variables-reference)
  - [BD_ Variables (CLI Tool Configuration)](#bd_-variables-cli-tool-configuration)
  - [BEADS_ Variables (Daemon & Advanced Settings)](#beads_-variables-daemon--advanced-settings)
  - [Integration Variables](#integration-variables)
- [Configuration Precedence](#configuration-precedence)
- [Usage Examples](#usage-examples)

---

## Environment Variables Reference

Environment variables can be used to configure Beads behavior without modifying the `.beads/config.yaml` file. All settings in `config.yaml` can also be set via environment variables with a `BD_` prefix.

### BD_ Variables (CLI Tool Configuration)

These variables control basic CLI behavior and output formatting.

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `BD_JSON` | boolean | `false` | Enable JSON output by default for all commands. Equivalent to passing `--json` flag. |
| `BD_NO_DAEMON` | boolean | `false` | Disable daemon for RPC communication, forcing direct database access. Equivalent to `no-daemon` in config. |
| `BD_NO_AUTO_FLUSH` | boolean | `false` | Disable automatic flushing of database to JSONL after mutations. Equivalent to `no-auto-flush` in config. |
| `BD_NO_AUTO_IMPORT` | boolean | `false` | Disable automatic import from JSONL when it's newer than database. Equivalent to `no-auto-import` in config. |
| `BD_DB` | path | auto-discover | Path to the SQLite database file. Overrides `db` setting in config. If not set, bd searches upward from current directory for `.beads/beads.db`. |
| `BD_ACTOR` | string | `$USER` | Default actor name for audit trails. Overrides `actor` setting in config. Used to track who made changes. |

### BEADS_ Variables (Daemon & Advanced Settings)

These variables control daemon behavior, synchronization, and advanced features.

#### Daemon Configuration

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `BEADS_AUTO_START_DAEMON` | boolean | `true` | Automatically start the daemon if not running. If `false`, requires manual `bd daemon --start`. |
| `BEADS_FLUSH_DEBOUNCE` | duration | `5s` | Debounce interval for auto-flush operations. Prevents excessive JSONL writes. Format: `5s`, `100ms`, `1m`, etc. |
| `BEADS_DAEMON_MODE` | enum | `poll` | Sync mode for daemon: `poll` (periodic polling) or `events` (filesystem events). |
| `BEADS_WATCHER_FALLBACK` | boolean | `true` | Fall back to polling mode if filesystem event watching fails. Only relevant when `BEADS_DAEMON_MODE=events`. |

#### Daemon Logging

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `BEADS_DAEMON_LOG_MAX_SIZE` | integer | `50` | Maximum daemon log file size in MB before rotation. |
| `BEADS_DAEMON_LOG_MAX_BACKUPS` | integer | `7` | Maximum number of old log files to retain. |
| `BEADS_DAEMON_LOG_MAX_AGE` | integer | `30` | Maximum number of days to keep old log files. |
| `BEADS_DAEMON_LOG_COMPRESS` | boolean | `true` | Compress rotated log files with gzip. |

#### Git Integration

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `BEADS_SYNC_BRANCH` | string | none | Git branch for beads commits when using `bd sync`. Can also be set in `config.yaml` as `sync-branch`. Important for team projects to ensure all clones use the same sync branch. |

#### Database Options

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `BEADS_DB` | path | `.beads/beads.db` | Alternative to `BD_DB`. Path to the SQLite database file. |

### Integration Variables

These variables are used for external service integrations.

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `ANTHROPIC_API_KEY` | string | none | Required for AI-powered issue compaction (`bd compact --auto`). Used for automatic issue summarization. |

---

## Configuration Precedence

Beads resolves configuration from multiple sources in the following order (highest to lowest priority):

1. **Command-line flags** (e.g., `--json`, `--no-daemon`)
2. **Environment variables** (e.g., `BD_JSON=true`)
3. **Config file** (`.beads/config.yaml`)
4. **Built-in defaults**

This means:
- CLI flags always override environment variables
- Environment variables override config file settings
- Config file settings override built-in defaults

---

## Usage Examples

### Basic CLI Configuration

```bash
# Always output JSON without passing --json flag
export BD_JSON=true
bd list

# Use custom actor name for audit trails
export BD_ACTOR="ci-bot"
bd update bd-123 --status done

# Use specific database path
export BD_DB="/path/to/custom/beads.db"
bd list
```

### Daemon Configuration

```bash
# Disable daemon for CI/CD environments
export BD_NO_DAEMON=true
bd list

# Prevent daemon from auto-starting
export BEADS_AUTO_START_DAEMON=false
bd daemon --start  # Must start manually

# Use event-driven sync mode (experimental)
export BEADS_DAEMON_MODE=events
export BEADS_WATCHER_FALLBACK=true

# Increase flush debounce for high-frequency operations
export BEADS_FLUSH_DEBOUNCE=10s
```

### Logging Configuration

```bash
# Increase log retention
export BEADS_DAEMON_LOG_MAX_SIZE=100
export BEADS_DAEMON_LOG_MAX_BACKUPS=14
export BEADS_DAEMON_LOG_MAX_AGE=60

# Disable log compression
export BEADS_DAEMON_LOG_COMPRESS=false
```

### Git Sync Configuration

```bash
# Set sync branch for team coordination
export BEADS_SYNC_BRANCH=beads-sync
bd sync
```

### Disable Auto-Import/Export

```bash
# Useful for manual control over JSONL synchronization
export BD_NO_AUTO_FLUSH=true
export BD_NO_AUTO_IMPORT=true

# Manual flush when needed
bd flush
```

### AI-Powered Features

```bash
# Enable AI compaction
export ANTHROPIC_API_KEY=sk-ant-...
bd compact --auto --dry-run
```

### CI/CD Environment

```bash
# Recommended settings for CI/CD pipelines
export BD_NO_DAEMON=true        # Direct DB access
export BD_JSON=true             # Machine-readable output
export BD_NO_AUTO_FLUSH=false   # Keep auto-flush enabled
export BD_ACTOR="github-actions" # Track automation

# Run beads commands
bd list --status open
bd create "CI found issue" -t bug
```

### Development Environment

```bash
# Recommended settings for local development
export BEADS_AUTO_START_DAEMON=true  # Convenience
export BEADS_FLUSH_DEBOUNCE=5s       # Default
export BD_JSON=false                  # Human-readable output
```

---

## Project-Specific Configuration

For this project (`mcp-agent-mail-rs`), the recommended environment setup is:

```bash
# Enable JSON output for AI agent integration
export BD_JSON=true

# Use standard sync branch (set in config.yaml)
# BEADS_SYNC_BRANCH is configured in .beads/config.yaml

# Keep daemon auto-start enabled for convenience
export BEADS_AUTO_START_DAEMON=true

# Use default debounce
export BEADS_FLUSH_DEBOUNCE=5s
```

These can be added to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.) or to a project-specific `.envrc` file if using [direnv](https://direnv.net/).

---

## Troubleshooting

### Database Issues

If you encounter database locking or corruption:

```bash
# Force direct DB access (bypass daemon)
export BD_NO_DAEMON=true
bd list
```

### Sync Issues

If JSONL and database are out of sync:

```bash
# Disable auto-import temporarily
export BD_NO_AUTO_IMPORT=true
bd flush  # Force export to JSONL

# Or re-import from JSONL
bd import .beads/issues.jsonl
```

### Daemon Issues

If the daemon is misbehaving:

```bash
# Check daemon logs
cat .beads/daemon.log

# Restart daemon
bd daemon --stop
bd daemon --start

# Or disable daemon entirely
export BEADS_AUTO_START_DAEMON=false
export BD_NO_DAEMON=true
```

---

## References

- [Beads GitHub Repository](https://github.com/steveyegge/beads)
- [Beads Configuration Documentation](https://github.com/steveyegge/beads/blob/main/docs/CONFIG.md)
- [Beads Daemon Documentation](https://github.com/steveyegge/beads/blob/main/docs/DAEMON.md)
- [Project AGENTS.md](../AGENTS.md) - Beads workflow for this project
- [Project CLAUDE.md](../CLAUDE.md) - Claude-specific beads usage
