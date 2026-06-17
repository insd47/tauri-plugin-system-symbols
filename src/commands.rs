use crate::{Path, Result};
use tauri::command;

#[command]
pub(crate) async fn get_symbol(symbol: String, size: f32) -> Result<Vec<Path>> {
    crate::platform::resolve_symbol(&symbol, size)
}
