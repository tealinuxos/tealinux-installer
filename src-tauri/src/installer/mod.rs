pub mod blueprint;
mod payload;
pub mod step;

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
use tea_arch_chroot_lib::chroot::bootloader::get_firmware_type;
use tea_arch_chroot_lib::chroot::*;
use tea_arch_chroot_lib::prechroot::*;
use tea_arch_chroot_lib::resource::FirmwareKind;

use tea_arch_chroot_lib::resource::MethodKind;
use tea_partition_generator::dual_boot_efi_mount;
use tea_partition_generator::os::Os;

pub use self::blueprint::BluePrint;
pub use self::blueprint::Bootloader;
pub use self::blueprint::Keyboard;
pub use self::blueprint::Partition;
pub use self::blueprint::Storage;

use tea_partition_generator::mkpart::Partgen;
use tea_partition_generator::tealinux_build_env;

fn wait() {
    let delay = Duration::from_secs(1);
    sleep(delay);
}

#[tauri::command]
pub async fn start_install(window: Window) {
    // Reading JSON into Blueprint

    let _build_env = tealinux_build_env::tealinux_build_env();
    if _build_env.unwrap() == tealinux_build_env::BuildType::Dev {
        // deny run start_install on dev env
        return ();
    }

    let _ = window.emit(
        "INSTALL",
        Payload {
            percentage: 0,
            message: "Starting installation".into(),
        },
    );

    wait();

    let mut blueprint = step::json::read_blueprint().expect("Failed when reading blueprint file");

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

    // Refreshing the blueprint to get new disk information
    // especially the one after being formatted
    blueprint = step::json::read_blueprint().expect("Failed when reading blueprint file");

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

    match step::boot::copy_kernel() {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit(
                "ERROR",
                self::payload::Error {
                    message: "Failed to copy kernel".into(),
                },
            );
            return ();
        }
    }

    // Copy microcode

    match step::microcode::copy_microcode().await {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit(
                "ERROR",
                self::payload::Error {
                    message: "Failed to copy microcode".into(),
                },
            );
            return;
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

    // return;

    if blueprint.storage.clone().unwrap().install_method != MethodKind::MANUAL {
        Os::append_swap_fstab(&blueprint.storage.clone().unwrap().into());
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

    if blueprint.storage.clone().unwrap().install_method == MethodKind::DUAL {
        // Os::append_swap_fstab(&blueprint.storage.clone().unwrap().into());
        Os::patch_grub_config_disable_os_probe(false);
    }

    let _ = window.emit(
        "INSTALL",
        Payload {
            percentage: 60,
            message: "Installing bootloader".into(),
        },
    );

    wait();

    // check whatever its dualboot auto partition & we need to locate where the "origin" efi partition is.
    if blueprint.storage.clone().unwrap().install_method == MethodKind::DUAL
        && get_firmware_type() == FirmwareKind::UEFI
    {
        dual_boot_efi_mount::dualboot_efi_mount_open(
            blueprint.storage.clone().unwrap().disk_path.unwrap(),
        );
    }

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

    // Remove installer desktop entry
    let _ = std::fs::remove_dir_all("/tealinux-mount/etc/skel/Desktop");

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
        Ok(_) => {
            if blueprint.account.as_ref().unwrap().autologin {
                match blueprint
                    .account
                    .as_ref()
                    .unwrap()
                    .set_cosmic_automatic_login()
                {
                    Ok(_) => (),
                    Err(_) => {
                        let _ = window.emit(
                            "ERROR",
                            self::payload::Error {
                                message: "Failed to configure user".into(),
                            },
                        );
                    }
                }
            }
        }
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
        None => Account::new("", "", "", "", false),
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

    // Remove installer autostart entry
    std::fs::remove_file("/tealinux-mount/etc/xdg/autostart/tealinux-installer.desktop")?;

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
