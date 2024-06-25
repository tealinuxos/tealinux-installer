use crate::read::get_read;
use crate::installer::BluePrint;
use std::fs::{ File, create_dir_all };
use std::io::Write;
use tea_arch_chroot_lib::resource::{ Locales, Timezones };
use crate::storage::get_storage;
use super::storage::filesystem::filesystem_list;
use std::path::Path;

#[tauri::command]
pub async fn get_read_json() -> String
{
    let read = get_read();

    let json = serde_json::to_string_pretty(&read);

    json.unwrap()
}

#[tauri::command]
pub async fn set_read_json()
{
    let json = self::get_read_json().await;

    let path = Path::new("/opt/tea-installer/");

    if !path.exists()
    {
        create_dir_all("/opt/tea-installer/");
    }

    let mut file = File::create("/opt/tea-installer/read.json").unwrap();

    file.write_fmt(format_args!("{}", json)).unwrap();
}

#[tauri::command]
pub async fn set_blueprint_json(json: String)
{
    let path = Path::new("/opt/tea-installer/");

    if !path.exists()
    {
        create_dir_all("/opt/tea-installer/");
    }

    let mut file = File::create("/opt/tea-installer/installer.json").unwrap();

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

#[tauri::command]
pub async fn get_filesystem_json() -> String
{
    let filesystem = filesystem_list();

    let json = serde_json::to_string_pretty(&filesystem).unwrap();

    json
}
