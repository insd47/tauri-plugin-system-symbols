const COMMANDS: &[&str] = &["get_symbol"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
