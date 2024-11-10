// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use serde::Deserialize;
use tauri::{ipc::{Request, Response}, Window};
mod commands;



#[tauri::command]
fn raw_request(request: Request<'_>) -> Response {
  println!("{request:?}");
  Response::new(include_bytes!("../.././README.md").to_vec())
}
#[derive(Deserialize)]
struct Person<'a> {
  name: &'a str,
  age: u8,
}

#[tauri::command]
fn command_arguments_struct(Person { name, age }: Person<'_>) {
  println!("received person struct with name: {name} | age: {age}")
}
#[tauri::command]
fn window_label(window: Window) {
  println!("window label: {}", window.label());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
          raw_request,
          command_arguments_struct,
          commands::simple_command,
          window_label,

        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
