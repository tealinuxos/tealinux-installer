pub mod blueprint;
mod payload;
mod step;

use std::fs::File;
use std::io::{ Error, BufReader };
use tea_arch_chroot_lib::prechroot::*;
use tea_arch_chroot_lib::chroot::*;
use super::read::online::Online;
use super::storage::umount_all_target;
use self::payload::Payload;
use tauri::Window;
use std::thread::sleep;
use std::time::Duration;
use duct::cmd;

pub use self::blueprint::BluePrint;
pub use self::blueprint::Partition;
pub use self::blueprint::Bootloader;
pub use self::blueprint::Keyboard;

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

    let blueprint = step::json::read_blueprint().expect("Failed when reading blueprint file");


    // Partitioning

    let _ = window.emit("INSTALL", Payload {
        percentage: 10,
        message: "Partitioning disk".into()
    });

    wait();

    match step::partitioning::partitioning(&blueprint)
    {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit("ERROR", self::payload::Error {
                message: "Error partitioning disk".into()
            });
            return ();
        }
    }


    // RSYNC system

    let _ = window.emit("INSTALL", Payload {
        percentage: 20,
        message: "Copying necessary files".into()
    });

    wait();

    match rsync::start_rsync().await
    {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit("ERROR", self::payload::Error {
                message: "Error when copying filesystem".into()
            });
            return ();
        }
    }


    // Copy kernel to new root

    match step::boot::copy_kernel()
    {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit("ERROR", self::payload::Error {
                message: "Failed to copy kernel".into()
            });
            return ();
        }
    }

    // Copy mkinitcpio preset to new root

    match step::boot::copy_mkinitcpio_preset()
    {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit("ERROR", self::payload::Error {
                message: "Failed copying mkinitcpio config".into()
            });
            return ();
        }
    }


    // Generate FSTAB

    let _ = window.emit("INSTALL", Payload {
        percentage: 30,
        message: "Generating Fstab".into()
    });

    wait();

    match fstab::generate_fstab().await
    {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit("ERROR", self::payload::Error {
                message: "Error generating fstab".into()
            });
            return ();
        }
    }


    // Chroot

    let _ = window.emit("INSTALL", Payload {
        percentage: 40,
        message: "Fixing keyring".into()
    });

    wait();

    match pacman::regenerate_pacman_key()
    {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit("ERROR", self::payload::Error {
                message: "Error populating pacman-key".into()
            });
            return ();
        }
    }


    // Mkinitcpio

    let _ = window.emit("INSTALL", Payload {
        percentage: 50,
        message: "Running mkinitcpio".into()
    });

    wait();

    match mkinitcpio::generate_initramfs("linux")
    {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit("ERROR", self::payload::Error {
                message: "Error generating initramfs".into()
            });
            return ();
        }
    }


    // Bootloader

    let _ = window.emit("INSTALL", Payload {
        percentage: 60,
        message: "Installing bootloader".into()
    });

    wait();

    match step::bootloader::install_bootloader(&blueprint)
    {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit("ERROR", self::payload::Error {
                message: "Failed to install bootloader".into()
            });
            return ();
        }
    }


    // Account

    let _ = window.emit("INSTALL", Payload {
        percentage: 70,
        message: "Creating account".into()
    });

    match blueprint.account.as_ref().unwrap().set_host()
    {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit("ERROR", self::payload::Error {
                message: "Failed to configure user".into()
            });
            return ();
        }
    }

    match blueprint.account.as_ref().unwrap().add_user()
    {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit("ERROR", self::payload::Error {
                message: "Failed to configure user".into()
            });
            return ();
        }
    }


    // Timezone and locale

    let _ = window.emit("INSTALL", Payload {
        percentage: 80,
        message: "Configuring timezone and locale".into()
    });

    wait();

    match blueprint.locale.unwrap().set_locale()
    {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit("ERROR", self::payload::Error {
                message: "Failed to configure locale".into()
            });
            return ();
        }
    }

    match blueprint.timezone.unwrap().generate_localtime()
    {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit("ERROR", self::payload::Error {
                message: "Failed to configure timezone".into()
            });
            return ();
        }
    }


    // Finishing up

    let _ = window.emit("INSTALL", Payload {
        percentage: 90,
        message: "Finishing up".into()
    });

    let _ = Account::remove_user("tea");

    let _ = post_install();

    // Umount previously mounted partition

    let _ = umount_all_target("/mnt");

    println!("Done");

    let _ = window.emit("INSTALL", Payload {
        percentage: 100,
        message: "Installation completed".into()
    });
}

fn post_install() -> Result<(), Error>
{
    cmd!("arch-chroot", "/mnt", "pacman", "-R", "--noconfirm", "tealinux-installer-git").run()?;
    std::fs::remove_file("/mnt/etc/xdg/autostart/tealinux-installer.desktop")?;
    // cmd!("arch-chroot", "/mnt", "machinectl", "shell", "gdm@", "/bin/bash", "-c", "'dbus-launch gsettings set org.gnome.login-screen logo /usr/share/icons/tealinux-logo.png'").run()?;

    Ok(())
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
