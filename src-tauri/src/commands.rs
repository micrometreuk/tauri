// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri::command;



#[command]
pub fn simple_command(the_argument: String) {
  println!("{the_argument}");
}
