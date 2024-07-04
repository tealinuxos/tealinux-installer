use tea_arch_chroot_lib::resource::FirmwareKind;
use tea_arch_chroot_lib::chroot::bootloader;
use crate::installer::BluePrint;
use std::io::Error;
use super::partitioning::{ get_boot_path, get_boot_mountpoint };

pub fn install_bootloader(blueprint: &BluePrint) -> Result<(), Error>
{
    if blueprint.bootloader.is_none()
    {
        return Ok(());
    }
    else
    {
        let firmware = blueprint.bootloader.as_ref().unwrap();

        match firmware.firmware_type
        {
            FirmwareKind::UEFI => {

                let path = if firmware.path.is_some()
                {
                    Some("/boot/efi".to_string())
                }
                else
                {
                    get_boot_mountpoint(blueprint)
                };

                bootloader::install_grub_bootloader(FirmwareKind::UEFI, None, path)?;
            }
            FirmwareKind::BIOS => {

                let path = if let Some(path) = firmware.path.clone()
                {
                    Some(path)
                }
                else
                {
                    get_boot_path(blueprint)
                };

                bootloader::install_grub_bootloader(FirmwareKind::BIOS, path, None)?;
            }
        }
    }

    Ok(())
}
