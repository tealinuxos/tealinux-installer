pub mod blueprint;
mod payload;
pub mod step;

use self::payload::Payload;
use super::read::online::Online;
use super::storage::umount_all_target;
use crate::read::get_read;
use duct::cmd;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use tauri::Emitter;
use tauri::Window;
use tea_arch_chroot_lib::chroot::bootloader::get_firmware_type;
use tea_arch_chroot_lib::chroot::shell;
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

/// Menjalankan rsync dari squashfs ke target, mem-parse output --progress secara
/// real-time, dan mengirim event INSTALL dengan persentase antara 20–29%.
fn run_rsync_with_progress(window: &Window) -> Result<(), Error> {
    let mut child = Command::new("rsync")
        .args([
            "-aAXHv",
            "--progress",
            "/run/archiso/airootfs/",
            "/tealinux-mount/",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    // Kumpulkan total file terlebih dahulu dari stdout
    // Format progress rsync per-file:
    //   "      1,234,567  45%    1.23MB/s    0:00:05"
    // Format ringkasan akhir berisi "to-chk=X/Y" — kita parse X dan Y
    // untuk menghitung berapa file sudah selesai → persentase 20-29%.

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| Error::new(std::io::ErrorKind::Other, "Cannot capture rsync stdout"))?;

    let reader = BufReader::new(stdout);

    // Regex sederhana: cocokkan "to-chk=DONE/TOTAL" dari baris ringkasan rsync
    // Contoh: "            3,815,123 100%    1.21MB/s    0:00:03 (xfr#42, to-chk=58/512)"
    let re_tochk = regex::Regex::new(r"to-chk=(\d+)/(\d+)")
        .map_err(|e| Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        if let Some(caps) = re_tochk.captures(&line) {
            // remaining = to-chk (berapa file yang belum disalin)
            let remaining: u64 = caps[1].parse().unwrap_or(0);
            let total: u64 = caps[2].parse().unwrap_or(1);

            if total > 0 {
                // done = total - remaining
                let done = total.saturating_sub(remaining);
                // Map ke rentang 20–29%
                // Saat done=0 → 20%, saat done=total → 29%
                let pct = 20 + ((done * 9) / total);
                let pct = pct.min(29) as u32;

                let _ = window.emit(
                    "INSTALL",
                    Payload {
                        percentage: pct,
                        message: format!("Copying files… ({}/{})", done, total),
                    },
                );
            }
        }
    }

    let status = child
        .wait()
        .map_err(|e| Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::new(
            std::io::ErrorKind::Other,
            format!("rsync exited with status: {}", status),
        ))
    }
}

#[tauri::command]
pub async fn start_install(window: Window) {
    let read = get_read();

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

    let mut blueprint = match step::json::read_blueprint() {
        Ok(bp) => bp,
        Err(_) => {
            let _ = window.emit(
                "ERROR",
                self::payload::Error {
                    message: "Failed to read blueprint file".into(),
                },
            );
            return ();
        }
    };

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

    match step::partitioning::partitioning(&blueprint).await {
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
    blueprint = match step::json::read_blueprint() {
        Ok(bp) => bp,
        Err(_) => {
            let _ = window.emit(
                "ERROR",
                self::payload::Error {
                    message: "Failed to read blueprint file".into(),
                },
            );
            return ();
        }
    };

    // RSYNC system

    let _ = window.emit(
        "INSTALL",
        Payload {
            percentage: 20,
            message: "Copying necessary files".into(),
        },
    );

    wait();

    match run_rsync_with_progress(&window) {
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

    // match step::microcode::copy_microcode().await {
    //     Ok(_) => (),
    //     Err(_) => {
    //         let _ = window.emit(
    //             "ERROR",
    //             self::payload::Error {
    //                 message: "Failed to copy microcode".into(),
    //             },
    //         );
    //         return;
    //     }
    // }

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

    // check whatever its dualboot auto partition & we need to locate where the "origin" efi partition is.
    if blueprint.storage.clone().unwrap().install_method == MethodKind::DUAL
        && get_firmware_type() == FirmwareKind::UEFI
    {
        dual_boot_efi_mount::dualboot_efi_mount_open(
            blueprint.storage.clone().unwrap().disk_path.unwrap(),
        );
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

    // if blueprint.storage.clone().unwrap().install_method != MethodKind::MANUAL {
    //     match Os::append_swap_fstab(&blueprint.storage.clone().unwrap().into())
    //     {
    //         Ok(_) => (),
    //         Err(_) => {
    //             let _ = window.emit(
    //                 "ERROR",
    //                 self::payload::Error {
    //                     message: "Error appending swap to fstab".into(),
    //                 },
    //             );
    //             return ();
    //         }
    //     }
    // }

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

    // this offset is location of original dualboot EFI mounter

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

    // Remove installer desktop entry
    let _ = std::fs::remove_dir_all("/tealinux-mount/etc/skel/Desktop");

    let account = match blueprint.account {
        Some(account) => account,
        None => Account::new("", "", "", "", false),
    };

    let _ = window.emit(
        "INSTALL",
        Payload {
            percentage: 70,
            message: "Creating account".into(),
        },
    );

    match account.set_host() {
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

    match account.add_user() {
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

    let desktop_environment = read.desktop_environment.name;

    match environment_specific_config(desktop_environment, &account) {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit(
                "ERROR",
                self::payload::Error {
                    message: "Failed applying specific desktop environment configuration.".into(),
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

    match post_install(&account) {
        Ok(_) => (),
        Err(_) => {
            let _ = window.emit(
                "ERROR",
                self::payload::Error {
                    message: "Failed to during post install step".into(),
                },
            );
            return ();
        }
    }

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

fn environment_specific_config(
    desktop_environment: String,
    account: &Account,
) -> Result<(), Error> {
    match desktop_environment.to_lowercase().as_ref() {
        "cosmic" => {
            if account.autologin {
                account.set_cosmic_automatic_login()?;
            }
        }
        "kde" => {
            if account.autologin {
                account.set_sddm_automatic_login()?;
            }
        }
        _ => (),
    }

    Ok(())
}

fn post_install(account: &Account) -> Result<(), Error> {
    // Change shell to fish
    shell::change_shell(&account.username, "/usr/bin/fish")?;

    // Remove liveuser
    let _ = Account::remove_user("liveuser");

    // Remove installer
    cmd!(
        "arch-chroot",
        "/tealinux-mount",
        "pacman",
        "-R",
        "--noconfirm",
        "tealinux-installer-git"
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
