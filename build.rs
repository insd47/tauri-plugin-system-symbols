const COMMANDS: &[&str] = &["get_symbol", "get_symbols"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
