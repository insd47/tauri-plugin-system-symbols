//! System symbol SVG path provider for Tauri.
//!
//! The plugin exposes a small command surface and keeps platform-specific
//! symbol extraction behind `platform` modules.

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

mod cache;
mod commands;
mod error;
mod models;
mod platform;

pub use error::{Error, Result};
pub use models::{FillRule, SvgPath, SvgSymbol};

/// Initializes the plugin. Register it with `tauri::Builder::plugin(init())`.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("system-symbols")
        .invoke_handler(tauri::generate_handler![
            commands::get_fluent_icons,
            commands::get_sf_symbols
        ])
        .setup(|app, _api| {
            app.manage(cache::SymbolCache::default());
            Ok(())
        })
        .build()
}
