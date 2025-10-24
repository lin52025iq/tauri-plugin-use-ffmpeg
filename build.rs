const COMMANDS: &[&str] = &["check", "download", "execute", "remove"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
