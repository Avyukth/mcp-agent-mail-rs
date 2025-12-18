# Contributing to MCP Agent Mail

We use strictly enforced quality gates.

## Pre-commit Hooks (prek)

We use `prek` (Rust-native pre-commit) to enforce:
1. `cargo fmt`
2. `cargo clippy`
3. `cargo audit`
4. `bd sync`

### Installation

1. Install prek:
   ```bash
   cargo install --locked prek
   ```

2. Install hooks:
   ```bash
   prek install
   ```

### Running Hooks Manually

```bash
prek run --all-files
```
