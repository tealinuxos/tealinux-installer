use tea_arch_chroot_lib::resource::FirmwareKind;
use tea_arch_chroot_lib::chroot::bootloader;
use crate::installer::BluePrint;
use std::io::Error;
use super::partitioning::{ get_boot_path, get_boot_mountpoint, get_bios_grub_path };

fn get_disk_name_from_partition(partition: Option<&str>) -> Option<String>
{
    if let Some(partition) = partition
    {
        let lsblk = duct::cmd!("lsblk", "-no", "PKNAME", partition)
            .pipe(duct::cmd!("uniq"))
            .read();

        if let Ok(path) = lsblk
        {
            Some(path.replace('\n', ""))
        }
        else
        {
            None
        }
    }
    else
    {
        None
    }
}

pub fn install_bootloader(blueprint: &BluePrint) -> Result<(), Error>
{
    if blueprint.bootloader.is_none()
    {
        return Ok(());
    }
    else
    {
        let firmware = blueprint.bootloader.as_ref().unwrap();
        let partition_table = blueprint.storage.as_ref().unwrap().partition_table.as_ref().unwrap();

        let bootloader_path = {

            let firmware_path = &firmware.path;

            if partition_table == "gpt"
            {
                if firmware.firmware_type == FirmwareKind::UEFI
                {
                    firmware_path.to_owned()
                }
                else
                {
                    get_disk_name_from_partition(firmware_path.as_deref())
                }
            }
            else
            {
                get_disk_name_from_partition(firmware_path.as_deref())
            }
        };

        match firmware.firmware_type
        {
            FirmwareKind::UEFI => {

                let path = if partition_table == "gpt"
                {
                    if bootloader_path.is_some()
                    {
                        Some("/boot/efi".to_string())
                    }
                    else
                    {
                        get_boot_mountpoint(blueprint)
                    }
                }
                else // should not happen
                {
                    get_boot_mountpoint(blueprint)
                };

                let path = path.map(|p| format!("/dev/{}", p));

                bootloader::install_grub_bootloader(FirmwareKind::UEFI, None, path)?;
            }
            FirmwareKind::BIOS => {

                let path = if partition_table == "gpt"
                {
                    if let Some(path) = bootloader_path
                    {
                        get_disk_name_from_partition(Some(&path))
                    }
                    else
                    {
                        let bios_grub_partition = get_bios_grub_path(blueprint);
                        get_disk_name_from_partition(bios_grub_partition.as_deref())
                    }
                }
                else
                {
                    get_boot_path(blueprint)
                };

                let path = path.map(|p| format!("/dev/{}", p));

                bootloader::install_grub_bootloader(FirmwareKind::BIOS, path, None)?;
            }
        }
    }

    Ok(())
}
