pub mod blueprint;
mod payload;
mod step;

use std::fs::File;
use std::io::BufReader;
use tea_arch_chroot_lib::prechroot::*;
use tea_arch_chroot_lib::chroot::*;
use super::read::online::Online;
use super::storage::umount_all_target;
use self::payload::Payload;
use tauri::Window;
use std::thread::sleep;
use std::time::Duration;

pub use self::blueprint::BluePrint;
pub use self::blueprint::Partition;

fn wait()
{
    let delay = Duration::from_secs(1);
    sleep(delay);
}

#[tauri::command]
pub async fn start_install(window: Window)
{
    // Reading JSON into Blueprint

    let _ = window.emit("INSTALL", Payload {
        percentage: 0,
        message: "Starting installation".into()
    });

    wait();

    let blueprint = step::json::read_blueprint().unwrap();


    // Partitioning

    let _ = window.emit("INSTALL", Payload {
        percentage: 10,
        message: "Partitioning disk".into()
    });

    wait();

    step::partitioning::partitioning(&blueprint).unwrap();


    // RSYNC system

    let _ = window.emit("INSTALL", Payload {
        percentage: 20,
        message: "Copying necessary files".into()
    });

    wait();

    rsync::start_rsync().await.unwrap();


    // Copy kernel to new root

    step::boot::copy_kernel().unwrap();

    // Copy mkinitcpio preset to new root

    step::boot::copy_mkinitcpio_preset().unwrap();


    // Generate FSTAB

    let _ = window.emit("INSTALL", Payload {
        percentage: 30,
        message: "Generating Fstab".into()
    });

    wait();

    fstab::generate_fstab().await.unwrap();


    // Chroot

    let _ = window.emit("INSTALL", Payload {
        percentage: 40,
        message: "Fixing keyring".into()
    });

    wait();

    pacman::regenerate_pacman_key().unwrap();


    // Mkinitcpio

    let _ = window.emit("INSTALL", Payload {
        percentage: 50,
        message: "Running mkinitcpio".into()
    });

    wait();

    mkinitcpio::generate_initramfs("linux").unwrap();


    // Bootloader

    let _ = window.emit("INSTALL", Payload {
        percentage: 60,
        message: "Installing bootloader".into()
    });

    wait();

    step::bootloader::install_bootloader(&blueprint).unwrap();


    // Account

    let _ = window.emit("INSTALL", Payload {
        percentage: 70,
        message: "Creating account".into()
    });

    blueprint.account.as_ref().unwrap().set_host().unwrap();
    blueprint.account.as_ref().unwrap().add_user().unwrap();


    // Timezone and locale

    let _ = window.emit("INSTALL", Payload {
        percentage: 80,
        message: "Configuring timezone and locale".into()
    });

    wait();

    blueprint.locale.unwrap().set_locale().unwrap();
    blueprint.timezone.unwrap().generate_localtime().unwrap();


    // Finishing up

    let _ = window.emit("INSTALL", Payload {
        percentage: 90,
        message: "Finishing up".into()
    });

    Account::remove_user("tea").unwrap();

    // Umount previously mounted partition

    umount_all_target("/mnt").unwrap();

    println!("Done");

    let _ = window.emit("INSTALL", Payload {
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
    let file = File::open("/opt/tea-installer/installer.json").unwrap();
    let reader = BufReader::new(file);

    let json: BluePrint = serde_json::from_reader(reader).unwrap();

    println!("{:#?}", json);
}
