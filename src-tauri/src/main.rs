// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![salam_sapa])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn salam_sapa(name: &str) -> String{
  format!("Hello World & Hello {}", name)
}