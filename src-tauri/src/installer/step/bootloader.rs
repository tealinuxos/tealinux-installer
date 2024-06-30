use tea_arch_chroot_lib::resource::FirmwareKind;
use tea_arch_chroot_lib::chroot::bootloader;
use crate::installer::BluePrint;
use std::io::Error;
use super::partitioning::{ get_boot_path, get_boot_mountpoint };

pub fn install_bootloader(blueprint: &BluePrint) -> Result<(), Error>
{
    let firmware = bootloader::get_firmware_type();

    match firmware
    {
        FirmwareKind::UEFI => {
            let destination = get_boot_mountpoint(blueprint);
            bootloader::install_grub_bootloader(FirmwareKind::UEFI, None, destination)?;
        }

        FirmwareKind::BIOS => {
            let destination = get_boot_path(blueprint);
            bootloader::install_grub_bootloader(FirmwareKind::BIOS, destination, None)?;
        }
    }

    Ok(())
}
