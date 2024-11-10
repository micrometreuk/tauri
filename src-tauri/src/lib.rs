// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::ipc::{Request, Response};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn raw_request(request: Request<'_>) -> Response {
  println!("{request:?}");
  Response::new(include_bytes!("../.././README.md").to_vec())
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![raw_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
