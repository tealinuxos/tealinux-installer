// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod installer;
mod read;
mod storage;
mod system;
mod utils;

use api::account::*;
use api::auto_partition::*;
use api::firmware::*;
use api::keyboard::*;
use api::locale::*;
use api::partition::*;
use api::timezone::*;
use api::*;
use installer::{is_online, print_json, start_install};
use system::reboot::reboot;
use system::spawn::*;
use users::get_current_uid;

// remove on release
use tauri::Manager;

fn main() {
    match get_current_uid() {
        0 => {
            tauri::Builder::default()
                .setup(|app| {
                    #[cfg(debug_assertions)] // only include this code on debug builds
                    {
                        let window = app.get_webview_window("main").unwrap();
                        window.open_devtools();
                        // window.close_devtools();
                    }
                    Ok(())
                })
                .invoke_handler(tauri::generate_handler![
                    get_read_json,
                    set_empty_blueprint,
                    blueprint_set_locale,
                    blueprint_set_timezone,
                    blueprint_set_account,
                    blueprint_set_partition,
                    blueprint_set_bootloader,
                    blueprint_set_keyboard,
                    set_read_json,
                    get_locale_json,
                    get_timezone_json,
                    start_install,
                    is_online,
                    get_filesystem_json,
                    print_json,
                    reboot,
                    get_read_from_opt,
                    get_blueprint_from_opt,
                    get_keyboard_json,
                    read_blueprint,
                    spawn_gparted,
                    spawn_terminal,
                    set_auto_config_partition,
                    get_disk_lists_key_val,
                    autogen_partition_select_disk
                ])
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
        _ => {
            println!("Run with sudo privileges!");
        }
    }
}
