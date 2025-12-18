//! File Handle Safety Patterns (PORT-2.3)
//!
//! This module documents the file safety patterns used throughout lib-core
//! to prevent file descriptor leaks.
//!
//! # Audit Summary
//!
//! All file operations in lib-core have been audited:
//!
//! | Module | Pattern | Status |
//! |--------|---------|--------|
//! | `git_store.rs` | `std::fs::write` (atomic) | ✅ Safe |
//! | `attachment.rs` | DB metadata only | ✅ No file I/O |
//! | `archive_lock.rs` | `tokio::fs` with RAII guard | ✅ Safe |
//! | `repo_cache.rs` | LRU eviction | ✅ FD bounded |
//!
//! # Best Practices
//!
//! ## 1. Prefer Atomic Operations
//!
//! ```rust,ignore
//! // ✅ GOOD: Atomic write (opens, writes, closes in one call)
//! std::fs::write(&path, content)?;
//!
//! // ❌ AVOID: Manual File handling (can leak on error)
//! let mut file = File::create(&path)?;
//! file.write_all(content.as_bytes())?;
//! // Implicit drop - timing uncertain
//! ```
//!
//! ## 2. Use Explicit Scopes
//!
//! ```rust,ignore
//! // ✅ GOOD: Explicit scope bounds file lifetime
//! fn process_file(path: &Path) -> Result<Data> {
//!     let data = {
//!         let file = File::open(path)?;
//!         let mut reader = BufReader::new(file);
//!         serde_json::from_reader(&mut reader)?
//!     }; // File explicitly dropped here
//!     Ok(data)
//! }
//! ```
//!
//! ## 3. Use RAII Guards for Resources
//!
//! See [`super::archive_lock::LockGuard`] for an example of RAII pattern
//! that ensures cleanup even on panic.
//!
//! ## 4. Use LRU Caching for Expensive Resources
//!
//! See [`super::repo_cache::RepoCache`] which limits open repositories
//! to prevent file descriptor exhaustion.
//!
//! # Git2 Repository Handles
//!
//! git2::Repository holds multiple file descriptors. Key safety patterns:
//!
//! - Use `RepoCache` to limit concurrent open repositories
//! - Don't hold `Repository` across `.await` points unnecessarily
//! - Let repositories drop naturally when work is complete
//!
//! # Async File Operations
//!
//! For async contexts, prefer `tokio::fs`:
//!
//! ```rust,ignore
//! use tokio::fs;
//!
//! // ✅ GOOD: Non-blocking, integrates with async runtime
//! fs::write(&path, content).await?;
//! let content = fs::read_to_string(&path).await?;
//! ```

/// Marker module for file safety documentation.
/// This module exists primarily for documentation purposes.
/// See parent module documentation for file safety patterns.
pub mod docs {}
