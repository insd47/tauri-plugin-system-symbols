const COMMANDS: &[&str] = &["get_fluent_icons", "get_sf_symbols"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
