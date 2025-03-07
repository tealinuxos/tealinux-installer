use crate::installer::BluePrint;
use crate::read::get_read;
use crate::storage::{ format, mount, umount, format_unallocated };
use crate::storage::btrfs::{ mount_subvolume, create_subvolume };
use duct::cmd;
use tea_arch_chroot_lib::resource::FirmwareKind;
use tea_partition_api_lib::GetDiskInformation;
use std::io::Error;
use std::fs::{create_dir, create_dir_all};

pub fn partitioning(blueprint: &BluePrint) -> Result<(), Error>
{
    // Ignore error if the mountpoint is not mounted
    let _ = cmd!("umount", "--recursive", "/tealinux-mount").run();

    let firmware_type = &blueprint.bootloader.as_ref().unwrap().firmware_type;
    let bootloader_path = &blueprint.bootloader.as_ref().unwrap().path;

    let max_number = blueprint.disk.as_ref().unwrap().iter().max_by_key(|partition| partition.number);
    let max_number = max_number.unwrap().number;
    let max_number = max_number + 1;

    for i in blueprint.disk.as_ref().unwrap().iter().to_owned().rev()
    {
        let mut i_path = i.path.clone();
        let i_format = i.format;
        let i_mountpoint = &i.mountpoint;
        let i_filesystem = &i.filesystem;
        let i_start = i.start;
        let i_end = i.end;
        let i_disk_path = &i.disk_path.as_ref().unwrap();

        if i_format && i_path.is_none() && i_filesystem.is_some()
        {
            let path = format_unallocated(i_disk_path, i_start, i_end, max_number , i_filesystem.as_ref().unwrap())?;
            i_path = path;
        }

        if i_format && i_filesystem.is_some() && i_path.is_some()
        {
            format(i_filesystem.as_ref().unwrap(), i_path.as_ref().unwrap())?;
        }

        if let Some(i_mountpoint) = i_mountpoint
        {
            if i_mountpoint.contains("swap")
            {
                cmd!("swapon", i_path.as_ref().unwrap()).run()?;
            }

            if i_mountpoint.contains("boot")
            {
                cmd!("mkdir", "--parents", format!("/tealinux-mount{}", i_mountpoint)).run()?;
                mount(i_path.as_ref().unwrap(), &format!("/tealinux-mount{}", i_mountpoint), None)?;
            }

            if i_mountpoint.eq("/")
            {
                if i_filesystem.as_ref().unwrap().eq("btrfs")
                {
                    mount(i_path.as_ref().unwrap(), "/tealinux-mount", None)?;

                    create_subvolume("/tealinux-mount/@")?;
                    create_subvolume("/tealinux-mount/@home")?;

                    umount("/tealinux-mount")?;

                    mount_subvolume("@", i_path.as_ref().unwrap(), "/tealinux-mount")?;

                    create_dir("/tealinux-mount/home")?;

                    mount_subvolume("@home", i_path.as_ref().unwrap(), "/tealinux-mount/home")?;
                }
                
                else
                {
                    mount(i_path.as_ref().unwrap(), "/tealinux-mount", None)?;
                }
            }
        }
    }

    match firmware_type
    {
        FirmwareKind::UEFI => {
            create_dir_all("/tealinux-mount/boot/efi")?;
            mount(&bootloader_path.as_ref().unwrap(), "/tealinux-mount/boot/efi", None)?;
        }

        FirmwareKind::BIOS => ()
    }

    Ok(())
}

pub fn get_boot_mountpoint(blueprint: &BluePrint) -> Option<String>
{
    let disk = &blueprint.disk;
    let mut boot_mountpoint = None;

    if let Some(disk) = disk
    {
        let boot_mountpoint_index = disk
            .iter()
            .position(|f| f.mountpoint.is_some() && f.mountpoint.as_ref().unwrap().contains("boot") );

        if let Some(index) = boot_mountpoint_index
        {
            boot_mountpoint = Some(disk[index].mountpoint.as_ref().unwrap());
        }
    }

    boot_mountpoint.cloned()
}

pub fn get_boot_path(blueprint: &BluePrint) -> Option<String>
{
    let disk = &blueprint.disk;
    let mut boot_path = None;

    if let Some(disk) = disk
    {
        let boot_path_index = disk
            .iter()
            .position(|f| f.path.is_some() && f.mountpoint.is_some() && f.mountpoint.as_ref().unwrap().contains("boot") );

        if let Some(index) = boot_path_index
        {
            boot_path = Some(disk[index].path.as_ref().unwrap());
        }
    }

    boot_path.cloned()
}
