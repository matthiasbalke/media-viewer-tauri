use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

#[cfg(test)]
use std::cell::RefCell;

static MANIFEST_LOCK: Mutex<()> = Mutex::new(());

#[cfg(test)]
thread_local! {
    static TEST_CACHE_DIR: RefCell<Option<PathBuf>> = RefCell::new(None);
}

/// Overrides the base cache directory for testing purposes.
#[cfg(test)]
pub(crate) fn set_test_cache_dir(path: Option<PathBuf>) {
    TEST_CACHE_DIR.with(|dir| {
        *dir.borrow_mut() = path;
    });
}

/// Returns the base cache directory: ~/.mv/thumbnails
/// If TEST_CACHE_DIR is set, uses that instead (for isolation in tests).
fn cache_base_dir() -> Result<PathBuf, String> {
    #[cfg(test)]
    {
        if let Some(test_dir) = TEST_CACHE_DIR.with(|dir| dir.borrow().clone()) {
            return Ok(test_dir);
        }
    }

    let home = dirs::home_dir().ok_or("Could not determine home directory")?;
    Ok(home.join(".mv").join("thumbnails"))
}

/// Returns the path to the manifest file.
fn manifest_path() -> Result<PathBuf, String> {
    Ok(cache_base_dir()?.join("manifest.json"))
}

/// Computes the hash string for a source path.
/// Normalizes the path first to ensure consistent hashes across platforms.
fn hash_for_path(source: &Path) -> String {
    let mut hasher = DefaultHasher::new();
    super::normalize_path(&source.to_string_lossy()).hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

/// Returns the path to the thumbnail for a given source file.
/// Format: ~/.mv/thumbnails/<size>/<hash>.jpg
pub fn thumbnail_path(source: &Path, size: u32) -> Result<PathBuf, String> {
    let base = cache_base_dir()?;
    let hash = hash_for_path(source);
    Ok(base.join(size.to_string()).join(format!("{}.jpg", hash)))
}

/// Returns true if the thumbnail is stale (source was modified after the thumbnail).
pub fn is_stale(source: &Path, thumbnail: &Path) -> bool {
    let source_mtime = match fs::metadata(source).and_then(|m| m.modified()) {
        Ok(t) => t,
        Err(_) => return true, // Can't read source → treat as stale
    };

    let thumb_mtime = match fs::metadata(thumbnail).and_then(|m| m.modified()) {
        Ok(t) => t,
        Err(_) => return true, // Thumbnail doesn't exist → stale
    };

    source_mtime > thumb_mtime
}

/// Creates the cache directory for the given thumbnail size.
pub fn ensure_cache_dir(size: u32) -> Result<PathBuf, String> {
    let cache_dir = cache_base_dir()?.join(size.to_string());

    if cache_dir.exists() {
        if !cache_dir.is_dir() {
            return Err(format!(
                "Cache path exists but is not a directory: {}",
                cache_dir.display()
            ));
        }
    } else {
        fs::create_dir_all(&cache_dir).map_err(|e| {
            format!(
                "Failed to create cache directory {}: {}",
                cache_dir.display(),
                e
            )
        })?;
    }

    // Verify we can write to the directory
    let test_file = cache_dir.join(".write_test");
    fs::write(&test_file, b"").map_err(|e| {
        format!(
            "Cache directory is not writable {}: {}",
            cache_dir.display(),
            e
        )
    })?;
    let _ = fs::remove_file(&test_file);

    Ok(cache_dir)
}

// --- Manifest management ---

/// Loads the manifest (hash → source_path).
fn load_manifest() -> Result<HashMap<String, String>, String> {
    let path = manifest_path()?;
    if !path.exists() {
        return Ok(HashMap::new());
    }
    let data = fs::read_to_string(&path).map_err(|e| format!("Failed to read manifest: {}", e))?;
    serde_json::from_str(&data).map_err(|e| format!("Failed to parse manifest: {}", e))
}

/// Saves the manifest to disk.
fn save_manifest(manifest: &HashMap<String, String>) -> Result<(), String> {
    let path = manifest_path()?;
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create manifest directory: {}", e))?;
    }
    let data = serde_json::to_string_pretty(manifest)
        .map_err(|e| format!("Failed to serialize manifest: {}", e))?;
    fs::write(&path, data).map_err(|e| format!("Failed to write manifest: {}", e))
}

/// Registers a thumbnail in the manifest after generation.
pub fn register_thumbnail(source: &Path) -> Result<(), String> {
    let _lock = MANIFEST_LOCK
        .lock()
        .map_err(|e| format!("Manifest lock error: {}", e))?;
    let hash = hash_for_path(source);
    let mut manifest = load_manifest()?;
    manifest.insert(hash, super::normalize_path(&source.to_string_lossy()));
    save_manifest(&manifest)
}

/// Deletes all thumbnails whose source path starts with the given prefix.
/// Used when a root directory is removed.
pub fn cleanup_for_prefix(prefix: &str) -> Result<u32, String> {
    let _lock = MANIFEST_LOCK
        .lock()
        .map_err(|e| format!("Manifest lock error: {}", e))?;
    let mut manifest = load_manifest()?;
    let base = cache_base_dir()?;

    let to_remove: Vec<String> = manifest
        .iter()
        .filter(|(_, source)| source.starts_with(prefix))
        .map(|(hash, _)| hash.clone())
        .collect();

    let mut removed = 0u32;
    for hash in &to_remove {
        // Try to delete all size variants
        if let Ok(entries) = fs::read_dir(&base) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let thumb = entry.path().join(format!("{}.jpg", hash));
                    if thumb.exists() {
                        let _ = fs::remove_file(&thumb);
                    }
                }
            }
        }
        manifest.remove(hash);
        removed += 1;
    }

    save_manifest(&manifest)?;
    Ok(removed)
}

/// Scans the manifest and deletes entries whose source file no longer exists.
pub fn cleanup_orphans() -> Result<u32, String> {
    let _lock = MANIFEST_LOCK
        .lock()
        .map_err(|e| format!("Manifest lock error: {}", e))?;
    let mut manifest = load_manifest()?;
    let base = cache_base_dir()?;

    let orphans: Vec<String> = manifest
        .iter()
        .filter(|(_, source)| !Path::new(source).exists())
        .map(|(hash, _)| hash.clone())
        .collect();

    let mut removed = 0u32;
    for hash in &orphans {
        // Delete all size variants
        if let Ok(entries) = fs::read_dir(&base) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let thumb = entry.path().join(format!("{}.jpg", hash));
                    if thumb.exists() {
                        let _ = fs::remove_file(&thumb);
                    }
                }
            }
        }
        manifest.remove(hash);
        removed += 1;
    }

    save_manifest(&manifest)?;
    Ok(removed)
}

/// Deletes the entire thumbnail cache directory.
pub fn delete_all() -> Result<(), String> {
    let _lock = MANIFEST_LOCK
        .lock()
        .map_err(|e| format!("Manifest lock error: {}", e))?;
    let base = cache_base_dir()?;
    if base.exists() {
        fs::remove_dir_all(&base).map_err(|e| format!("Failed to delete cache dir: {}", e))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::thread;
    use std::time::Duration;
    use tempfile::tempdir;

    /// Helper function to create an isolated test environment
    /// Automatically cleans up the global override when dropped.
    struct TestEnvGuard {
        pub temp_dir: tempfile::TempDir,
    }

    impl Drop for TestEnvGuard {
        fn drop(&mut self) {
            set_test_cache_dir(None);
        }
    }

    fn setup_test_env() -> TestEnvGuard {
        let temp_dir = tempdir().expect("Failed to create temp test directory");
        set_test_cache_dir(Some(temp_dir.path().to_path_buf()));
        TestEnvGuard { temp_dir }
    }

    #[test]
    fn test_hash_for_path_deterministic() {
        // Same logical path should yield same hash
        let hash1 = hash_for_path(&PathBuf::from("/foo/bar/image.jpg"));
        let hash2 = hash_for_path(&PathBuf::from("/foo/bar/image.jpg"));
        assert_eq!(hash1, hash2, "Hashes should be deterministic");
    }

    #[test]
    fn test_hash_for_path_different_files() {
        // Different files yield different hashes
        let hash1 = hash_for_path(&PathBuf::from("/foo/bar/image.jpg"));
        let hash3 = hash_for_path(&PathBuf::from("/foo/bar/other.jpg"));
        assert_ne!(hash1, hash3, "Different paths should have different hashes");
    }

    #[test]
    fn test_hash_for_path_cross_platform() {
        // Cross-platform logic (Windows vs Unix slash)
        let hash_win = hash_for_path(&PathBuf::from("C:\\foo\\image.jpg"));
        let hash_unix = hash_for_path(&PathBuf::from("C:/foo/image.jpg"));
        assert_eq!(
            hash_win, hash_unix,
            "Path normalization should ensure identical hashes"
        );
    }

    #[test]
    fn test_ensure_cache_dir_creates_directory() {
        let _env = setup_test_env();

        let size = 128; // Use 128 instead of 256 to avoid clashes with older tests if dirty
        let cache_dir = ensure_cache_dir(size).expect("Failed to create cache dir");

        assert!(cache_dir.exists());
        assert!(cache_dir.is_dir());
        assert!(cache_dir.ends_with("128"));
    }

    #[test]
    fn test_manifest_starts_empty() {
        let _env = setup_test_env();

        // Initially empty
        let initial_manifest = load_manifest().unwrap();
        assert!(initial_manifest.is_empty(), "Manifest should start empty");
    }

    #[test]
    fn test_register_thumbnail_adds_to_manifest() {
        let _env = setup_test_env();

        // Add an entry
        let test_path = PathBuf::from("/test/source/image.jpg");
        register_thumbnail(&test_path).expect("Failed to register thumbnail");

        // Load and verify
        let updated_manifest = load_manifest().unwrap();
        assert_eq!(updated_manifest.len(), 1);

        let hash = hash_for_path(&test_path);
        assert_eq!(
            updated_manifest.get(&hash).unwrap(),
            "/test/source/image.jpg"
        );
    }

    #[test]
    fn test_is_stale_when_thumbnail_missing() {
        let _env = setup_test_env();
        let base_dir = _env.temp_dir.path();

        let source_path = base_dir.join("source.jpg");
        let thumb_path = base_dir.join("thumb.jpg");

        // Create source file
        File::create(&source_path).unwrap();

        // 1. Thumbnail missing -> should be stale
        assert!(
            is_stale(&source_path, &thumb_path),
            "Missing thumbnail should be stale"
        );
    }

    #[test]
    fn test_is_stale_when_thumbnail_newer() {
        let _env = setup_test_env();
        let base_dir = _env.temp_dir.path();

        let source_path = base_dir.join("source.jpg");
        let thumb_path = base_dir.join("thumb.jpg");

        // Create source file
        File::create(&source_path).unwrap();

        // Create thumbnail file immediately
        // (Wait slightly to ensure mtime ticks forward for older filesystems)
        thread::sleep(Duration::from_millis(50));
        File::create(&thumb_path).unwrap();

        // 2. Thumbnail newer than source -> not stale
        assert!(
            !is_stale(&source_path, &thumb_path),
            "Newer thumbnail should not be stale"
        );
    }

    #[test]
    fn test_is_stale_when_source_newer() {
        let _env = setup_test_env();
        let base_dir = _env.temp_dir.path();

        let source_path = base_dir.join("source.jpg");
        let thumb_path = base_dir.join("thumb.jpg");

        // Create source file
        File::create(&source_path).unwrap();

        // Create thumbnail file immediately
        thread::sleep(Duration::from_millis(50));
        File::create(&thumb_path).unwrap();

        // Update source file to make it newer
        thread::sleep(Duration::from_millis(50));
        fs::write(&source_path, b"updated").unwrap();

        // 3. Source newer than thumbnail -> stale
        assert!(
            is_stale(&source_path, &thumb_path),
            "Newer source should make thumbnail stale"
        );
    }

    // Must remove MV_TEST_CACHE_DIR after tests to avoid cross-contamination in other threads,
    // though `cargo test` runs in parallel, which makes full env var isolation tricky.
    // Usually tests run locally will be fine.
}
