# GitHub Pages Deployment

This guide explains how to deploy a static archive of the MCP Agent Mail UI to GitHub Pages, allowing you to share your agent mail history as a read-only website.

## Overview

The static deployment feature creates a completely standalone version of the web UI that:

- **Runs offline** - No backend server required
- **Is read-only** - Cannot modify data, only view
- **Shows a demo banner** - Indicates static mode to users
- **Preserves full history** - All messages, threads, and activity exported

### Build Isolation

The deployment uses two completely separate build artifacts:

| Build | Output | Data Source | Use Case |
|-------|--------|-------------|----------|
| Embedded | `build/` | `/api/*` endpoints | Normal server operation |
| Static | `build-static/` | Bundled JSON files | GitHub Pages hosting |

The builds are tree-shaken at compile time - the embedded build has zero static-provider code, and the static build has zero api-provider code.

## Quick Start

```bash
# 1. Build the release binary
make build-release

# 2. Export data and build static site
make build-web-static

# 3. Deploy to GitHub Pages
make deploy-github-pages GITHUB_PAGES_REPO=my-mail-archive
```

## Detailed Steps

### 1. Export Database to JSON

Export your current database to static JSON files:

```bash
./target/release/mcp-agent-mail share export static-data \
    --output crates/services/web-ui/static/data
```

Options:
- `--output <dir>` - Output directory (required)
- `--scrub <mode>` - Privacy scrubbing: `none`, `standard`, `aggressive`
- `--limit <n>` - Maximum messages to export
- `--project <slug>` - Export only one project

Exported files:
- `meta.json` - Export metadata and timestamp
- `projects.json` - All projects
- `agents.json` - All agents by project
- `messages.json` - All messages
- `threads.json` - Thread summaries
- `dashboard.json` - Pre-computed dashboard stats
- `activity.json` - Recent activity
- `archive.json` - Git archive commits

### 2. Build Static Site

Build the SvelteKit app for static deployment:

```bash
cd crates/services/web-ui
VITE_DATA_MODE=static VITE_BUILD_MODE=static bun run build
```

Or use the Makefile target:

```bash
make build-web-static
```

This creates `build-static/` with all files needed for GitHub Pages.

### 3. Deploy to GitHub Pages

Deploy using the CLI:

```bash
./target/release/mcp-agent-mail share deploy github-pages \
    --repo mail-archive \
    --build-dir crates/services/web-ui/build-static \
    --create-repo
```

Options:
- `--repo <name>` - Repository name (required)
- `--owner <user>` - GitHub username/org (defaults to authenticated user)
- `--build-dir <path>` - Directory to deploy
- `--bundle <path>` - Alternative: ZIP file to deploy
- `--custom-domain <domain>` - Set custom domain
- `--token <token>` - GitHub token (or set `GITHUB_TOKEN` env var)
- `--create-repo` - Create repository if it doesn't exist
- `--private` - Make repository private

### Using the Makefile

The Makefile provides convenient targets:

```bash
# Export data only
make export-static-data

# Build static site (includes export)
make build-web-static

# Deploy (includes build and export)
make deploy-github-pages GITHUB_PAGES_REPO=mail-archive

# Full workflow from scratch
make full-deploy-github-pages GITHUB_PAGES_REPO=mail-archive

# With custom domain
make deploy-github-pages \
    GITHUB_PAGES_REPO=mail-archive \
    GITHUB_PAGES_DOMAIN=mail.example.com
```

## Static Mode Features

When viewing the static site, users will see:

1. **Demo Mode Banner** - Yellow banner at the top indicating static mode
2. **Export Timestamp** - Shows when the archive was created
3. **Download Link** - Option to download the raw archive
4. **Disabled Actions** - Compose, send, and delete buttons are hidden

## Verification

After implementation, verify bundle isolation:

```bash
# Embedded build should NOT contain "static-provider"
grep -r "static-provider" build/ && echo "FAIL" || echo "PASS"

# Static build should NOT contain "api-provider"
grep -r "api-provider" build-static/ && echo "FAIL" || echo "PASS"
```

## Rate Limiting

The GitHub API has rate limits. The deployment:

- Uploads files in batches of 10
- Adds 1-second delays between batches
- Shows progress during upload

For large sites (100+ files), deployment may take a few minutes.

## Custom Domains

To use a custom domain:

1. Set the `--custom-domain` flag when deploying
2. Configure DNS to point to GitHub Pages:
   - Add a CNAME record: `mail.example.com` → `username.github.io`
3. GitHub will automatically handle HTTPS

## Troubleshooting

### "Bundle not found" error
Ensure you've run the build step before deploying:
```bash
make build-web-static
```

### "GitHub token required" error
Set the `GITHUB_TOKEN` environment variable:
```bash
export GITHUB_TOKEN=ghp_your_token_here
```

### Files not updating
The GitHub Pages CDN may cache files. Wait a few minutes or try a hard refresh.

### Large file failures
GitHub has a 100MB file size limit. For large archives, consider:
- Splitting across multiple repositories
- Using Git LFS for large files
- Reducing the export limit with `--limit`

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Build Pipeline                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────┐              ┌─────────────────────────┐   │
│  │  bun run build  │              │  bun run build:static   │   │
│  │  (embedded)     │              │  (github-pages)         │   │
│  └────────┬────────┘              └───────────┬─────────────┘   │
│           │                                   │                 │
│           ▼                                   ▼                 │
│  ┌─────────────────┐              ┌─────────────────────────┐   │
│  │  build/         │              │  build-static/          │   │
│  │  - api-provider │              │  - static-provider      │   │
│  │  (NO static)    │              │  - data/*.json          │   │
│  └────────┬────────┘              │  (NO api-client)        │   │
│           │                       └───────────┬─────────────┘   │
│           ▼                                   │                 │
│  ┌─────────────────┐                          ▼                 │
│  │  rust-embed     │              ┌─────────────────────────┐   │
│  │  (lib-server)   │              │  GitHub Pages           │   │
│  └─────────────────┘              └─────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

## Related Commands

```bash
# Create a ZIP archive for manual sharing
mcp-agent-mail archive save --output archive.zip

# Verify an existing archive
mcp-agent-mail share verify --manifest manifest.json

# List available MCP tools
mcp-agent-mail tools
```
