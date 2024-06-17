use crate::read::get_read;
use crate::installer::BluePrint;
use std::fs::File;
use std::io::Write;
use tea_arch_chroot_lib::resource::{ Locales, Timezones };
use crate::storage::get_storage;
// use std::os::unix::fs::FileExt;

#[tauri::command]
pub async fn get_read_json() -> String
{
    let read = get_read();

    let json = serde_json::to_string_pretty(&read);

    json.unwrap()
}

#[tauri::command]
pub async fn set_blueprint_json(json: String)
{
    // let json: BluePrint = serde_json::from_str(&json).unwrap();
    let mut file = File::create("/opt/installer.json").unwrap();
    file.write_fmt(format_args!("{}", json)).unwrap();
}

#[tauri::command]
pub async fn get_locale_json() -> String
{
    Locales::list_json()
}

#[tauri::command]
pub async fn get_timezone_json() -> String
{
    Timezones::list_json()
}

#[tauri::command]
pub async fn get_storage_json() -> String
{
    let partition = get_storage().await;
    let json = serde_json::to_string_pretty(&partition).unwrap();

    json
}
