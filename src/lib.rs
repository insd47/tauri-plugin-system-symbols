//! System symbol SVG path provider for Tauri.
//!
//! The plugin exposes a small command surface and keeps platform-specific
//! symbol extraction behind `platform` modules.

pub use error::{Error, Result};
pub use models::{FillRule, Path};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

mod commands;
mod error;
mod models;
mod platform;

/// Initializes the plugin. Register it with `tauri::Builder::plugin(init())`.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("system-symbols")
        .invoke_handler(tauri::generate_handler![commands::get_symbol])
        .build()
}

/// Resolves one platform system symbol into SVG path data.
///
/// On Windows, the value is resolved as a Segoe Fluent Icons glyph or
/// codepoint. On macOS, the value is resolved as a copied SF Symbol character.
pub fn get_symbol(symbol: impl AsRef<str>, size: f32) -> Result<Vec<Path>> {
    platform::resolve_symbol(symbol.as_ref(), size)
}
