use super::storage::filesystem::filesystem_list;
use crate::installer::BluePrint;
use crate::read::{get_read, Read};
use crate::utils::error_handler;
use crate::utils::error_handler::ErrorHandler;
use os::get_other_os;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::{BufReader, BufWriter, Error, Write};
use std::path::Path;
use tea_arch_chroot_lib::chroot::bootloader::get_firmware_type;
use tea_arch_chroot_lib::resource::Keyboard;

pub mod account;
pub mod auto_partition;
pub mod firmware;
pub mod keyboard;
pub mod locale;
pub mod os;
pub mod partition;
pub mod storage;
pub mod timezone;

#[tauri::command]
#[specta::specta]
pub async fn get_read_json() -> Result<String, error_handler::ErrorHandlerEmitter> {
    let read = get_read();

    let json: Result<String, serde_json::Error> = serde_json::to_string_pretty(&read);
    match json {
        Ok(json_ok) => return Ok(json_ok),
        Err(json_err) => {
            let ret = error_handler::InternalErrorHandlerEmitter::new(
                "Error",
                "Failed to convert system info into json",
                error_handler::ErrorHandlerServerity::LOW,
                Box::new(json_err),
            );

            // ret.record_log();
            return Err(ret.get_return());
        }
    }

    // json.unwrap()
}

#[tauri::command]
#[specta::specta]
pub async fn get_read_from_opt() -> String {
    let path = "/opt/tea-installer/read.json";

    match read_to_string(path) {
        Ok(json) => json,
        Err(_) => {
            self::set_read_json().await;
            read_to_string(path).unwrap()
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_blueprint_from_opt() -> String {
    let path = "/opt/tea-installer/installer.json";

    match read_to_string(path) {
        Ok(json) => json,
        Err(_) => {
            let blueprint = BluePrint {
                account: None,
                locale: None,
                timezone: None,
                storage: None,
                bootloader: None,
                keyboard: None,
            };
            let mut file = File::create("/opt/tea-installer/installer.json").unwrap();

            let json = serde_json::to_string_pretty(&blueprint).unwrap();

            let _ = file.write_fmt(format_args!("{}", json)).unwrap();

            File::create("/opt/tea-installer/installer.json").unwrap();

            read_to_string(path).unwrap()
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn set_read_json() -> Result<(), error_handler::ErrorHandlerEmitter> {
    let json = self::get_read_json().await;

    match json {
        Ok(json_ok) => {
            let path = Path::new("/opt/tea-installer/");

            if !path.exists() {
                let ret = create_dir_all("/opt/tea-installer/");
                if let Err(ret_err) = ret {
                    let err_ret = error_handler::InternalErrorHandlerEmitter::new(
                        "Error",
                        "Failed to create a directory at /opt/tea-installer/",
                        error_handler::ErrorHandlerServerity::HIGH,
                        Box::new(ret_err),
                    );

                    return Err(err_ret.get_return());
                }
            }

            let file = File::create("/opt/tea-installer/read.json");

            match file {
                Ok(mut file_ok) => {
                    file_ok.write_fmt(format_args!("{}", json_ok)).unwrap();
                    return Ok(());
                }
                Err(file_err) => {
                    let err_ret = error_handler::InternalErrorHandlerEmitter::new(
                        "Error",
                        "Failed to create a file /opt/tea-installer/read.json",
                        error_handler::ErrorHandlerServerity::HIGH,
                        Box::new(file_err),
                    );

                    return Err(err_ret.get_return());
                }
            }
        }
        Err(json_err) => {
            return Err(json_err);
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn set_empty_blueprint() -> Result<(), error_handler::ErrorHandlerEmitter> {
    let path = Path::new("/opt/tea-installer/");

    if !path.exists() {
        let ret = create_dir_all("/opt/tea-installer/");
        if let Err(ret_err) = ret {
            let err_ret = error_handler::InternalErrorHandlerEmitter::new(
                "Error",
                "Failed to create a directory at /opt/tea-installer/",
                error_handler::ErrorHandlerServerity::HIGH,
                Box::new(ret_err),
            );

            return Err(err_ret.get_return());
        }
        // create_dir_all("/opt/tea-installer/").unwrap();
    }

    let file = File::create("/opt/tea-installer/installer.json");

    match file {
        Ok(mut file_ok) => {
            let blueprint = BluePrint {
                account: None,
                locale: None,
                timezone: None,
                storage: None,
                bootloader: None,
                keyboard: None,
            };

            let blueprint_json = serde_json::to_string_pretty(&blueprint).unwrap();

            file_ok
                .write_fmt(format_args!("{}", blueprint_json))
                .unwrap();
        }
        Err(file_err) => {
            let err_ret = error_handler::InternalErrorHandlerEmitter::new(
                "Error",
                "Failed to create a file /opt/tea-installer/installer.json",
                error_handler::ErrorHandlerServerity::HIGH,
                Box::new(file_err),
            );

            return Err(err_ret.get_return());
        }
    }
    return Ok(());
}

pub fn get_blueprint() -> Result<BluePrint, Error> {
    let file = File::open("/opt/tea-installer/installer.json")?;
    let reader = BufReader::new(file);

    let blueprint: BluePrint = serde_json::from_reader(reader)?;

    Ok(blueprint)
}

pub fn write_blueprint(blueprint: BluePrint) -> Result<(), Error> {
    let blueprint = serde_json::to_string_pretty(&blueprint)?;

    let file = File::create("/opt/tea-installer/installer.json")?;
    let mut writer = BufWriter::new(file);

    let _ = writer.write_fmt(format_args!("{}", blueprint));

    Ok(())
}

pub fn write_read(read: Read) -> Result<(), Error> {
    let read = serde_json::to_string_pretty(&read)?;

    let file = File::create("/opt/tea-installer/read.json")?;
    let mut writer = BufWriter::new(file);

    let _ = writer.write_fmt(format_args!("{}", read));

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_filesystem_json() -> String {
    let filesystem = filesystem_list();

    serde_json::to_string_pretty(&filesystem).unwrap()
}

#[tauri::command]
#[specta::specta]
pub async fn read_blueprint() {
    let blueprint = self::get_blueprint().unwrap();

    println!("{:#?}", blueprint);
}

#[tauri::command]
#[specta::specta]
pub async fn get_other_os_json() -> String {
    let other_os = get_other_os();

    if let Some(o) = other_os {
        serde_json::to_string_pretty(&o).unwrap()
    } else {
        String::from("{}")
    }
}
