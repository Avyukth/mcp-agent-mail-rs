//! LRU Repository Cache (PORT-2.1)
//!
//! Thread-safe LRU cache for git repositories to prevent file descriptor exhaustion.
//! NIST Control: SC-5 (DoS Protection)

use crate::error::Result;
use git2::Repository;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::debug;

/// Thread-safe LRU cache for git repositories
///
/// Limits open file descriptors by evicting least-recently-used repos.
/// Default capacity: 8 repos (each repo can use 10-50 FDs)
pub struct RepoCache {
    cache: Arc<Mutex<LruCache<PathBuf, Arc<Mutex<Repository>>>>>,
    capacity: usize,
}

impl RepoCache {
    /// Create cache with specified capacity
    ///
    /// # Arguments
    /// * `capacity` - Max repos to cache (default 8, each uses ~10-50 FDs)
    ///
    /// # Panics
    /// Panics if capacity is 0
    #[allow(clippy::expect_used)] // Capacity 0 is a programmer error, not runtime
    pub fn new(capacity: usize) -> Self {
        let cap = NonZeroUsize::new(capacity).expect("cache capacity must be > 0");

        Self {
            cache: Arc::new(Mutex::new(LruCache::new(cap))),
            capacity,
        }
    }

    /// Get or open repository at path
    ///
    /// Thread-safe with interior mutability via `Arc<Mutex<Repository>>`
    pub async fn get(&self, path: &Path) -> Result<Arc<Mutex<Repository>>> {
        // Canonicalize path (io::Error converts to Error::Io via #[from])
        let canonical = path.canonicalize()?;

        let mut cache = self.cache.lock().await;

        // Check cache first (updates LRU order)
        if let Some(repo) = cache.get(&canonical) {
            debug!(path = %canonical.display(), "Cache hit");
            return Ok(Arc::clone(repo));
        }

        // Open new repository (git2::Error converts to Error::Git2 via #[from])
        debug!(path = %canonical.display(), "Cache miss, opening repo");
        let repo = Repository::open(&canonical)?;

        let repo = Arc::new(Mutex::new(repo));

        // Insert into cache (may evict LRU entry)
        if cache.len() >= self.capacity {
            if let Some((evicted_path, _evicted_repo)) = cache.pop_lru() {
                debug!(path = %evicted_path.display(), "Evicted repo from cache");
                // Repository dropped here, releasing file handles
            }
        }

        cache.put(canonical.clone(), Arc::clone(&repo));

        Ok(repo)
    }

    /// Non-blocking check if path is cached
    ///
    /// Returns `None` if cache lock is held (doesn't block)
    pub fn peek(&self, path: &Path) -> Option<bool> {
        let canonical = path.canonicalize().ok()?;

        // Try non-blocking lock
        match self.cache.try_lock() {
            Ok(cache) => Some(cache.contains(&canonical)),
            Err(_) => None, // Lock held, can't check
        }
    }

    /// Get cached repo without opening (for fast paths)
    pub async fn get_if_cached(&self, path: &Path) -> Option<Arc<Mutex<Repository>>> {
        let canonical = path.canonicalize().ok()?;
        let cache = self.cache.lock().await;
        cache.peek(&canonical).cloned()
    }

    /// Current cache size
    pub async fn len(&self) -> usize {
        self.cache.lock().await.len()
    }

    /// Check if cache is empty
    pub async fn is_empty(&self) -> bool {
        self.len().await == 0
    }

    /// Clear all cached repos (for testing/shutdown)
    pub async fn clear(&self) {
        let mut cache = self.cache.lock().await;
        cache.clear();
        debug!("Repo cache cleared");
    }
}

impl Default for RepoCache {
    fn default() -> Self {
        Self::new(8) // 8 repos * ~50 FDs = ~400 FDs (well under ulimit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    async fn create_test_repo() -> (TempDir, PathBuf) {
        let dir = TempDir::new().expect("create temp dir");
        let path = dir.path().to_path_buf();
        Repository::init(&path).expect("init repo");
        (dir, path)
    }

    #[tokio::test]
    async fn test_cache_hit() {
        let cache = RepoCache::new(2);
        let (_dir, path) = create_test_repo().await;

        // First access - cache miss
        let _repo1 = cache.get(&path).await.expect("get repo");

        // Second access - cache hit
        let _repo2 = cache.get(&path).await.expect("get repo again");

        assert_eq!(cache.len().await, 1);
    }

    #[tokio::test]
    async fn test_lru_eviction() {
        let cache = RepoCache::new(2); // Capacity 2

        let (_dir1, path1) = create_test_repo().await;
        let (_dir2, path2) = create_test_repo().await;
        let (_dir3, path3) = create_test_repo().await;

        // Fill cache
        cache.get(&path1).await.expect("get path1");
        cache.get(&path2).await.expect("get path2");
        assert_eq!(cache.len().await, 2);

        // Add third - should evict path1 (LRU)
        cache.get(&path3).await.expect("get path3");
        assert_eq!(cache.len().await, 2);

        // path1 should be evicted
        assert!(cache.get_if_cached(&path1).await.is_none());
        assert!(cache.get_if_cached(&path2).await.is_some());
        assert!(cache.get_if_cached(&path3).await.is_some());
    }

    #[tokio::test]
    async fn test_peek_nonblocking() {
        let cache = RepoCache::new(2);
        let (_dir, path) = create_test_repo().await;

        // Not cached yet
        assert_eq!(cache.peek(&path), Some(false));

        // Add to cache
        cache.get(&path).await.expect("get repo");

        // Now cached
        assert_eq!(cache.peek(&path), Some(true));
    }

    #[tokio::test]
    async fn test_clear() {
        let cache = RepoCache::new(2);
        let (_dir, path) = create_test_repo().await;

        cache.get(&path).await.expect("get repo");
        assert_eq!(cache.len().await, 1);

        cache.clear().await;
        assert!(cache.is_empty().await);
    }
}
