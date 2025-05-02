// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod read;
mod utils;
mod storage;
mod installer;
mod system;

use api::*;
use api::locale::*;
use api::timezone::*;
use api::keyboard::*;
use api::account::*;
use api::partition::*;
use api::firmware::*;
use system::reboot::reboot;
use system::spawn::*;
use users::get_current_uid;
use installer::{ start_install, is_online, print_json };
use tauri::RunEvent;

fn main()
{
    match get_current_uid()
    {
        0 => {
            tauri::Builder::default()
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
                    spawn_terminal
                ])
                .build(tauri::generate_context!())
                .expect("error while running tauri application")
                .run(|_app_handle, _event| {
                    match _event {
                        RunEvent::Exit => {
                            duct::cmd!("xhost", "-si:localuser:root").run().unwrap();
                        },
                        _ => ()
                    }
                });
        }
        _ => {
            duct::cmd!("xhost", "si:localuser:root").run().unwrap();
            sudo::escalate_if_needed().unwrap();
        }
    }
}
