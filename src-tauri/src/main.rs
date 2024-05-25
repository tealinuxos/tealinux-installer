// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod installer;
use installer::partitions;
use installer::read::*;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        partitions::get_disk_info,
        battery::get_battery_percentage,
        lspci::lspci,
        memory::get_memory_capacity,
        memory::get_memory_free,
        model::get_model_number,
        model::get_brand,
        os::get_other_os,
        storage::get_storage_capacity,
        storage::get_partitions,
        storage::get_flags,
        storage::get_mountpoints,
        storage::get_filesystem
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
