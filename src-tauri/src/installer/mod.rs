pub mod blueprint;
mod payload;

use std::fs::File;
use std::io::BufReader;
use tea_arch_chroot_lib::prechroot::*;
use tea_arch_chroot_lib::chroot::*;
use tea_arch_chroot_lib::resource::FirmwareKind;
use super::read::online::Online;
use super::storage::{ mount, format, umount_all };
use super::storage::filesystem::filesystem_list;
use self::payload::Payload;
use duct::cmd;
use tauri::Window;
use std::thread::sleep;
use std::time::Duration;

pub use self::blueprint::BluePrint;

fn wait()
{
    let delay = Duration::from_secs(1);
    sleep(delay);
}

#[tauri::command]
pub async fn start_install(window: Window)
{
    // Begin installation

    window.emit("INSTALL", Payload {
        percentage: 0,
        message: "Reading JSON".into()
    });

    wait();

    let file = File::open("/opt/installer.json").unwrap();
    let reader = BufReader::new(file);

    let json: BluePrint = serde_json::from_reader(reader).unwrap();

    window.emit("INSTALL", Payload {
        percentage: 10,
        message: "Starting installation".into()
    });

    wait();

    let mut grub_efi_directory: Option<String> = None;
    let mut grub_disk_path: Option<String> = None;

    println!("{:#?}", json);

    // Partitioning

    window.emit("INSTALL", Payload {
        percentage: 20,
        message: "Partitioning disk".into()
    });

    wait();

    for i in json.partition.iter().to_owned().rev()
    {
        let i_path = &i.path;
        let i_format = &i.format;
        let i_mountpoint = &i.mountpoint;

        if i_format.is_some()
        {
            format(i_format.clone().unwrap(), i_path.clone()).await.unwrap();
        }
        if i_mountpoint.is_some()
        {
            if i_mountpoint.clone().unwrap().contains("boot")
            {
                grub_efi_directory = Some(i.mountpoint.clone().unwrap().to_string());
                grub_disk_path = Some(i.path.clone().to_string());

                cmd!("mkdir", format!("/mnt{}", i_mountpoint.clone().unwrap())).run().unwrap();
                mount(i_path.clone(), format!("/mnt{}", &i_mountpoint.clone().unwrap()), None).await.unwrap();
            }
            else
            {
                mount(i_path.clone(), format!("/mnt{}", &i_mountpoint.clone().unwrap()), None).await.unwrap();
            }
        }
    }

    println!("Partitioning done");


    // PreChroot

    window.emit("INSTALL", Payload {
        percentage: 30,
        message: "Copying necessary files".into()
    });

    wait();

    rsync::start_rsync().await.unwrap();

    println!("Rsync done");

    window.emit("INSTALL", Payload {
        percentage: 40,
        message: "Generating Fstab".into()
    });

    wait();

    fstab::generate_fstab().await.unwrap();

    // Chroot

    window.emit("INSTALL", Payload {
        percentage: 50,
        message: "Generating keyring".into()
    });

    wait();

    pacman::regenerate_pacman_key().unwrap();

    window.emit("INSTALL", Payload {
        percentage: 60,
        message: "Installing necessary packages".into()
    });

    wait();

    pacman::refresh_database().unwrap();
    pacman::install_package(vec!["mkinitcpio", "linux"]).unwrap();

    window.emit("INSTALL", Payload {
        percentage: 70,
        message: "Updating packages".into()
    });

    wait();

    pacman::update_packages();

    println!("Pacman done");

    window.emit("INSTALL", Payload {
        percentage: 80,
        message: "Installing bootloader".into()
    });

    wait();

    match bootloader::get_firmware_type()
    {
        FirmwareKind::UEFI => {
            bootloader::install_grub_bootloader(FirmwareKind::UEFI, None, grub_efi_directory).unwrap();
        }
        FirmwareKind::BIOS => {
            bootloader::install_grub_bootloader(FirmwareKind::BIOS, grub_disk_path, None).unwrap();
        }
    }

    println!("Bootloader Done");

    window.emit("INSTALL", Payload {
        percentage: 90,
        message: "Finishing up".into()
    });

    wait();

    Account::remove_user("tea").unwrap();
    json.locale.set_locale().unwrap();
    json.timezone.generate_localtime().unwrap();
    json.account.set_host().unwrap();
    json.account.add_user().unwrap();

    println!("Locale, timezone, account, Done");

    umount_all().unwrap();

    println!("Done");

    window.emit("INSTALL", Payload {
        percentage: 100,
        message: "Installation completed".into()
    });
}

#[tauri::command]
pub async fn is_online() -> bool
{
    let online = Online::new();

    online.status()
}

#[tauri::command]
pub async fn print_json()
{
    let file = File::open("/opt/installer.json").unwrap();
    let reader = BufReader::new(file);

    let json: BluePrint = serde_json::from_reader(reader).unwrap();

    println!("{:#?}", json);
}
