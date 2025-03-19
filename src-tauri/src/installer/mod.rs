pub mod blueprint;
mod payload;
mod step;

use step::mkpart;

use self::payload::Payload;
use super::read::online::Online;
use super::storage::umount_all_target;
use duct::cmd;
use std::fs::File;
use std::io::{BufReader, Error};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use tauri::Emitter;
use tauri::Window;
use tea_arch_chroot_lib::chroot::*;
use tea_arch_chroot_lib::prechroot::*;

pub use self::blueprint::BluePrint;
pub use self::blueprint::Bootloader;
pub use self::blueprint::Keyboard;
pub use self::blueprint::Storage;

fn wait() {
    let delay = Duration::from_secs(1);
    sleep(delay);
}

#[tauri::command]
pub async fn start_install(window: Window) {
    // Reading JSON into Blueprint

    let _ = window.emit(
        "INSTALL",
        Payload {
            percentage: 0,
            message: "Starting installation".into(),
        },
    );

    wait();

    let blueprint = step::json::read_blueprint().expect("Failed when reading blueprint file");

    // doing partition stuff, make one IF AUTOGEN IS SET
    mkpart::Partgen::do_dangerous_task_on(
        &blueprint._reserved.selected_format_disk,
        blueprint.disk.clone().unwrap(),
    );

    if !Path::exists(Path::new("/tealinux-mount")) {
        match std::fs::create_dir("/tealinux-mount/") {
            Ok(_) => (),
            Err(_) => {
                let _ = window.emit(
                    "ERROR",
                    self::payload::Error {
                        message: "Error when creating mount directory".into(),
                    },
                );
                return ();
            }
        }
    }

    wait();

    // Partitioning

    let _ = window.emit(
        "INSTALL",
        Payload {
            percentage: 10,
            message: "Partitioning disk".into(),
        },
    );

    wait();

    match step::partitioning::partitioning(&blueprint) {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit(
                "ERROR",
                self::payload::Error {
                    message: "Error partitioning disk".into(),
                },
            );
            return ();
        }
    }

    // RSYNC system

    let _ = window.emit(
        "INSTALL",
        Payload {
            percentage: 20,
            message: "Copying necessary files".into(),
        },
    );

    wait();

    match rsync::start_rsync().await {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit(
                "ERROR",
                self::payload::Error {
                    message: "Error when copying filesystem".into(),
                },
            );
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

    // Copy microcode

    match step::microcode::copy_microcode().await
    {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit("ERROR", self::payload::Error {
                message: "Failed to copy microcode".into()
            });
            return
        }
    }

    // Copy mkinitcpio preset to new root

    match step::boot::copy_mkinitcpio_preset() {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit(
                "ERROR",
                self::payload::Error {
                    message: "Failed copying mkinitcpio config".into(),
                },
            );
            return ();
        }
    }

    // Generate FSTAB

    let _ = window.emit(
        "INSTALL",
        Payload {
            percentage: 30,
            message: "Generating Fstab".into(),
        },
    );

    wait();

    match fstab::generate_fstab().await {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit(
                "ERROR",
                self::payload::Error {
                    message: "Error generating fstab".into(),
                },
            );
            return ();
        }
    }

    // Chroot

    let _ = window.emit(
        "INSTALL",
        Payload {
            percentage: 40,
            message: "Fixing keyring".into(),
        },
    );

    wait();

    match pacman::regenerate_pacman_key() {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit(
                "ERROR",
                self::payload::Error {
                    message: "Error populating pacman-key".into(),
                },
            );
            return ();
        }
    }

    // Mkinitcpio

    let _ = window.emit(
        "INSTALL",
        Payload {
            percentage: 50,
            message: "Running mkinitcpio".into(),
        },
    );

    wait();

    match mkinitcpio::generate_initramfs("linux") {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit(
                "ERROR",
                self::payload::Error {
                    message: "Error generating initramfs".into(),
                },
            );
            return ();
        }
    }

    // Bootloader

    let _ = window.emit(
        "INSTALL",
        Payload {
            percentage: 60,
            message: "Installing bootloader".into(),
        },
    );

    wait();

    match step::bootloader::install_bootloader(&blueprint) {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit(
                "ERROR",
                self::payload::Error {
                    message: "Failed to install bootloader".into(),
                },
            );
            return ();
        }
    }

    // Account

    let _ = window.emit(
        "INSTALL",
        Payload {
            percentage: 70,
            message: "Creating account".into(),
        },
    );

    match blueprint.account.as_ref().unwrap().set_host() {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit(
                "ERROR",
                self::payload::Error {
                    message: "Failed to configure user".into(),
                },
            );
            return ();
        }
    }

    match blueprint.account.as_ref().unwrap().add_user() {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit(
                "ERROR",
                self::payload::Error {
                    message: "Failed to configure user".into(),
                },
            );
            return ();
        }
    }

    // Timezone and locale

    let _ = window.emit(
        "INSTALL",
        Payload {
            percentage: 80,
            message: "Configuring timezone and locale".into(),
        },
    );

    wait();

    match blueprint.locale.unwrap().set_locale() {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit(
                "ERROR",
                self::payload::Error {
                    message: "Failed to configure locale".into(),
                },
            );
            return ();
        }
    }

    match blueprint.timezone.unwrap().generate_localtime() {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit(
                "ERROR",
                self::payload::Error {
                    message: "Failed to configure timezone".into(),
                },
            );
            return ();
        }
    }

    // Finishing up

    let _ = window.emit(
        "INSTALL",
        Payload {
            percentage: 90,
            message: "Finishing up".into(),
        },
    );

    let _ = Account::remove_user("liveuser");

    let account = match blueprint.account {
        Some(account) => account,
        None => Account::new("", "", "", ""),
    };

    let _ = post_install(account);

    // Umount previously mounted partition

    let _ = umount_all_target("/tealinux-mount");

    // Remove mount directory

    let _ = std::fs::remove_dir("/tealinux-mount");

    println!("Done");

    let _ = window.emit(
        "INSTALL",
        Payload {
            percentage: 100,
            message: "Installation completed".into(),
        },
    );
}

fn post_install(account: Account) -> Result<(), Error> {
    cmd!(
        "arch-chroot",
        "/tealinux-mount",
        "pacman",
        "-R",
        "--noconfirm",
        "tealinux-installer-git"
    )
    .run()?;
    std::fs::remove_file("/tealinux-mount/etc/xdg/autostart/tealinux-installer.desktop")?;
    // cmd!("arch-chroot", "/tealinux-mount", "machinectl", "shell", "gdm@", "/bin/bash", "-c", "'dbus-launch gsettings set org.gnome.login-screen logo /usr/share/icons/tealinux-logo.png'").run()?;
    //

    cmd!(
        "arch-chroot",
        "/tealinux-mount",
        "chsh",
        "--shell",
        "/usr/bin/fish",
        account.username
    )
    .run()?;

    Ok(())
}

#[tauri::command]
pub async fn is_online() -> bool {
    let online = Online::new();

    online.status()
}

#[tauri::command]
pub async fn print_json() {
    let file = File::open("/opt/tea-installer/installer.json").unwrap();
    let reader = BufReader::new(file);

    let json: BluePrint = serde_json::from_reader(reader).unwrap();

    println!("{:#?}", json);
}
