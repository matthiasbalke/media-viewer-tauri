use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

static MANIFEST_LOCK: Mutex<()> = Mutex::new(());

/// Returns the base cache directory: ~/.mv/thumbnails
fn cache_base_dir() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("Could not determine home directory")?;
    Ok(home.join(".mv").join("thumbnails"))
}

/// Returns the path to the manifest file.
fn manifest_path() -> Result<PathBuf, String> {
    Ok(cache_base_dir()?.join("manifest.json"))
}

/// Computes the hash string for a source path.
fn hash_for_path(source: &Path) -> String {
    let mut hasher = DefaultHasher::new();
    source.to_string_lossy().hash(&mut hasher);
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
    manifest.insert(hash, source.to_string_lossy().to_string());
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
