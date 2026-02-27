use super::cache;
use super::normalize_path;
use serde::Serialize;
use std::path::Path;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::Semaphore;

const THUMBNAIL_SIZE: u32 = 512;
const MAX_WORKERS: usize = 4;

const SUPPORTED_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "tif", "ico",
];

const SUPPORTED_MIME_TYPES: &[&str] = &[
    "image/jpeg",
    "image/png",
    "image/gif",
    "image/bmp",
    "image/webp",
    "image/tiff",
    "image/vnd.microsoft.icon",
    "image/x-icon",
];

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ThumbnailUpdate {
    path: String,
    status: String,
    thumbnail_path: Option<String>,
    session_id: u64,
}

pub struct ThumbnailService;

impl ThumbnailService {
    /// Returns true if the file is a supported image format by checking its magic bytes.
    fn is_supported(path: &Path) -> bool {
        // First try to infer type from the file contents (magic bytes)
        // If that fails (e.g. permission error, file doesn't exist), fall back to checking the extension.
        if let Ok(kind_opt) = infer::get_from_path(path) {
            if let Some(kind) = kind_opt {
                if SUPPORTED_MIME_TYPES.contains(&kind.mime_type()) {
                    return true;
                }
            }
        }

        // Fallback for files infer might miss but image crate might support
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| SUPPORTED_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
            .unwrap_or(false)
    }

    /// Generates a thumbnail for a single file.
    /// Returns the thumbnail path on success.
    fn generate_single(source: &Path) -> Result<String, String> {
        let thumb_path = cache::thumbnail_path(source, THUMBNAIL_SIZE)?;

        // Check if cached thumbnail is still valid
        if thumb_path.exists() && !cache::is_stale(source, &thumb_path) {
            return Ok(thumb_path.to_string_lossy().to_string());
        }

        // Ensure cache directory exists
        cache::ensure_cache_dir(THUMBNAIL_SIZE)?;

        // Open and resize the image
        let img = image::open(source).map_err(|e| {
            format!(
                "Failed to open image {}: {}",
                normalize_path(&source.to_string_lossy()),
                e
            )
        })?;

        let thumbnail = img.thumbnail(THUMBNAIL_SIZE, THUMBNAIL_SIZE);

        // Save as JPEG
        thumbnail
            .save(&thumb_path)
            .map_err(|e| format!("Failed to save thumbnail: {}", e))?;

        // Register in manifest for cleanup tracking
        cache::register_thumbnail(source)?;

        Ok(thumb_path.to_string_lossy().to_string())
    }

    /// Generates thumbnails for all media files in a directory.
    /// Emits `thumbnail-update` events to the frontend as each file is processed.
    pub async fn generate_for_dir(
        dir: String,
        session_id: u64,
        app_handle: AppHandle,
    ) -> Result<(), String> {
        let dir_path = Path::new(&dir);
        if !dir_path.is_dir() {
            return Err(format!("Not a directory: {}", dir));
        }

        // Read directory entries
        let entries: Vec<_> = std::fs::read_dir(dir_path)
            .map_err(|e| format!("Failed to read directory: {}", e))?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .collect();

        let semaphore = Arc::new(Semaphore::new(MAX_WORKERS));
        let mut handles = Vec::new();

        for entry in entries {
            let path = entry.path();
            let app = app_handle.clone();
            let sem = semaphore.clone();

            let handle = tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();

                let path_str = normalize_path(&path.to_string_lossy());

                if !Self::is_supported(&path) {
                    let _ = app.emit(
                        "thumbnail-update",
                        ThumbnailUpdate {
                            path: path_str,
                            status: "unsupported".to_string(),
                            thumbnail_path: None,
                            session_id,
                        },
                    );
                    return;
                }

                // Run blocking image work off the async thread
                let result = tokio::task::spawn_blocking({
                    let path = path.clone();
                    move || Self::generate_single(&path)
                })
                .await;

                match result {
                    Ok(Ok(thumb_path)) => {
                        let _ = app.emit(
                            "thumbnail-update",
                            ThumbnailUpdate {
                                path: path_str,
                                status: "ready".to_string(),
                                thumbnail_path: Some(normalize_path(&thumb_path)),
                                session_id,
                            },
                        );
                    }
                    Ok(Err(err)) => {
                        eprintln!("Thumbnail error for {}: {}", path_str, err);
                        let _ = app.emit(
                            "thumbnail-update",
                            ThumbnailUpdate {
                                path: path_str,
                                status: "error".to_string(),
                                thumbnail_path: None,
                                session_id,
                            },
                        );
                    }
                    Err(err) => {
                        eprintln!("Task join error for {}: {}", path_str, err);
                        let _ = app.emit(
                            "thumbnail-update",
                            ThumbnailUpdate {
                                path: path_str,
                                status: "error".to_string(),
                                thumbnail_path: None,
                                session_id,
                            },
                        );
                    }
                }
            });

            handles.push(handle);
        }

        // Wait for all tasks to complete
        for handle in handles {
            let _ = handle.await;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_is_supported_valid_extensions() {
        // Valid extensions
        assert!(ThumbnailService::is_supported(&PathBuf::from("image.jpg")));
        assert!(ThumbnailService::is_supported(&PathBuf::from("image.JPEG")));
        assert!(ThumbnailService::is_supported(&PathBuf::from("image.png")));
        assert!(ThumbnailService::is_supported(&PathBuf::from("photo.gif")));
        assert!(ThumbnailService::is_supported(&PathBuf::from("PIC.BMP")));
        assert!(ThumbnailService::is_supported(&PathBuf::from("test.webp")));
        assert!(ThumbnailService::is_supported(&PathBuf::from("test.tiff")));
        assert!(ThumbnailService::is_supported(&PathBuf::from("test.ico")));
    }

    #[test]
    fn test_is_supported_invalid_extensions() {
        // Invalid extensions
        assert!(!ThumbnailService::is_supported(&PathBuf::from("doc.txt")));
        assert!(!ThumbnailService::is_supported(&PathBuf::from("doc.pdf")));
        assert!(!ThumbnailService::is_supported(&PathBuf::from(
            "image.heic"
        ))); // Not currently in SUPPORTED_EXTENSIONS
        assert!(!ThumbnailService::is_supported(&PathBuf::from("video.mp4")));
    }

    #[test]
    fn test_is_supported_edge_cases() {
        // Edge cases
        assert!(!ThumbnailService::is_supported(&PathBuf::from(
            "no_extension_file"
        )));
        assert!(!ThumbnailService::is_supported(&PathBuf::from(
            ".hidden_no_ext"
        )));
    }
}
