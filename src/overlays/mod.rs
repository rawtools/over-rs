
/// Overlay files basename
const BASENAME: &str = "over";

/// Overlay files extensions
const EXTENSIONS: &[&str] = &["yml", "yaml", "toml", "json"];

pub mod overlay;
pub mod repository;

pub use overlay::Overlay;
pub use repository::Repository;
