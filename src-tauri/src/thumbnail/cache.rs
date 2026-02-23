use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

/// Returns the base cache directory: ~/.mv/thumbnails
fn cache_base_dir() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("Could not determine home directory")?;
    Ok(home.join(".mv").join("thumbnails"))
}

/// Returns the path to the thumbnail for a given source file.
/// Format: ~/.mv/thumbnails/<size>/<hash>.jpg
pub fn thumbnail_path(source: &Path, size: u32) -> Result<PathBuf, String> {
    let base = cache_base_dir()?;
    let mut hasher = DefaultHasher::new();
    source.to_string_lossy().hash(&mut hasher);
    let hash = format!("{:016x}", hasher.finish());

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
/// Returns the cache directory path or an error if it can't be created.
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
