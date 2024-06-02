// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod read;

use api::get_read_json;
use users::get_current_uid;

fn main()
{
    match get_current_uid()
    {
        0 => {
            tauri::Builder::default()
                .invoke_handler(tauri::generate_handler![
                    get_read_json
                ])
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
        _ => {
            println!("Run with sudo:\nsudo -E bunx tauri dev");
        }
    }
}
