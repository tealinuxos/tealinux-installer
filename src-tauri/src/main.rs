// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod read;
mod utils;
mod storage;
mod installer;

use api::*;
use users::get_current_uid;
use installer::{ start_install, is_online };

fn main()
{
    match get_current_uid()
    {
        0 => {
            tauri::Builder::default()
                .invoke_handler(tauri::generate_handler![
                    get_read_json,
                    set_blueprint_json,
                    get_locale_json,
                    get_timezone_json,
                    start_install,
                    is_online,
                    get_storage_json
                ])
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
        _ => {
            println!("Run with sudo:\nsudo -E bunx tauri dev");
        }
    }
}
