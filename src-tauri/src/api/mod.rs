use crate::read::get_read;
use crate::installer::BluePrint;
use std::fs::{ File, create_dir_all, read_to_string };
use std::io::{ Write, Read, BufReader, BufWriter, Error };
use super::storage::filesystem::filesystem_list;
use std::path::Path;
use tea_arch_chroot_lib::chroot::bootloader::get_firmware_type;
use tea_arch_chroot_lib::resource::Keyboard;

pub mod locale;
pub mod timezone;
pub mod keyboard;
pub mod account;
pub mod firmware;
pub mod storage;
pub mod partition;

#[tauri::command]
pub async fn get_read_json() -> String
{
    let read = get_read();

    let json = serde_json::to_string_pretty(&read);

    json.unwrap()
}

#[tauri::command]
pub async fn get_read_from_opt() -> String
{
    let path = "/opt/tea-installer/read.json";

    match read_to_string(path)
    {
        Ok(json) => json,
        Err(_) => {
            self::set_read_json().await;
            read_to_string(path).unwrap()
        }
    }
}

#[tauri::command]
pub async fn get_blueprint_from_opt() -> String
{
    let path = "/opt/tea-installer/installer.json";

    match read_to_string(path)
    {
        Ok(json) => json,
        Err(_) => {

            let blueprint = BluePrint { account: None, locale: None, timezone: None, storage: None, bootloader: None, keyboard: None };
            let mut file = File::create("/opt/tea-installer/installer.json").unwrap();

            let json = serde_json::to_string_pretty(&blueprint).unwrap();

            let _ = file.write_fmt(format_args!("{}", json)).unwrap();

            File::create("/opt/tea-installer/installer.json").unwrap();

            read_to_string(path).unwrap()
        }
    }
}

#[tauri::command]
pub async fn set_read_json()
{
    let json = self::get_read_json().await;

    let path = Path::new("/opt/tea-installer/");

    if !path.exists()
    {
        create_dir_all("/opt/tea-installer/").unwrap();
    }

    let mut file = File::create("/opt/tea-installer/read.json").unwrap();

    file.write_fmt(format_args!("{}", json)).unwrap();
}

#[tauri::command]
pub async fn set_empty_blueprint()
{
    let path = Path::new("/opt/tea-installer/");

    if !path.exists()
    {
        create_dir_all("/opt/tea-installer/").unwrap();
    }

    let mut file = File::create("/opt/tea-installer/installer.json").unwrap();

    let blueprint = BluePrint { account: None, locale: None, timezone: None, storage: None, bootloader: None, keyboard: None };

    let blueprint_json = serde_json::to_string_pretty(&blueprint).unwrap();

    file.write_fmt(format_args!("{}", blueprint_json)).unwrap();
}

pub fn get_blueprint() -> Result<BluePrint, Error>
{
    let file = File::open("/opt/tea-installer/installer.json")?;
    let reader = BufReader::new(file);

    let blueprint: BluePrint = serde_json::from_reader(reader)?;

    Ok(blueprint)
}

pub fn write_blueprint(blueprint: BluePrint) -> Result<(), Error>
{
    let blueprint = serde_json::to_string_pretty(&blueprint)?;
    
    let file = File::create("/opt/tea-installer/installer.json")?;
    let mut writer = BufWriter::new(file);

    let _ = writer.write_fmt(format_args!("{}", blueprint));

    Ok(())
}

#[tauri::command]
pub async fn get_filesystem_json() -> String
{
    let filesystem = filesystem_list();

    serde_json::to_string_pretty(&filesystem).unwrap()
}

#[tauri::command]
pub async fn read_blueprint()
{
    let blueprint = self::get_blueprint().unwrap();

    println!("{:#?}", blueprint);
}
