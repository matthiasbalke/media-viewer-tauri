mod cache;
mod service;

pub use cache::{cleanup_for_prefix, cleanup_orphans};
pub use service::ThumbnailService;

/// Normalizes a file path to use forward slashes.
/// This ensures consistent paths across platforms.
pub(crate) fn normalize_path(path: &str) -> String {
    path.replace('\\', "/")
}
