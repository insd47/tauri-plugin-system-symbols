use crate::models::Symbol;
use crate::Result;
use tauri::command;

#[command]
pub(crate) async fn get_symbol(symbol: String, size: f32) -> Result<Symbol> {
    crate::platform::resolve_symbol(&symbol, size)
}
