use crate::installer::{BluePrint, Partition};
use crate::read::get_read;
use crate::storage::{ create_new_partition_table, create_new_partition_table_with_partition, format, format_unallocated, mount, umount };
use crate::storage::btrfs::{ mount_subvolume, create_subvolume };
use duct::cmd;
use tea_arch_chroot_lib::resource::FirmwareKind;
use tea_partition_api_lib::GetDiskInformation;
use std::io::{Error, ErrorKind};
use std::fs::{create_dir, create_dir_all};

fn force_umount()
{
    // Ignore error if the mountpoint is not mounted
    let _ = cmd!("umount", "--recursive", "/tealinux-mount").run();
}

pub fn partitioning(blueprint: &BluePrint) -> Result<(), Error>
{
    let mut result = Ok(());

    if let Some(storage) = blueprint.storage.as_ref()
    {
        if storage.new_partition_table
        {
            result = partitioning_new_partition_table(blueprint);
        }
        else if storage.layout_changed
        {
            result = partitioning_layout_changed(blueprint);
        }
        else
        {
            result = partitioning_layout_preserved(blueprint);
        }
    }

    result
}

fn partitioning_new_partition_table(blueprint: &BluePrint) -> Result<(), Error>
{
    force_umount();

    let disk_path = &blueprint.storage.as_ref().unwrap().disk_path;
    let partition_table = &blueprint.storage.as_ref().unwrap().partition_table;
    let partitions = blueprint.storage.as_ref().unwrap().partitions.clone();

    let firmware_type = &blueprint.bootloader.as_ref().unwrap().firmware_type;
    let bootloader_path = &blueprint.bootloader.as_ref().unwrap().path;

    create_new_partition_table(disk_path.as_ref().unwrap(), partition_table.as_ref().unwrap())?;

    println!("create new partition table done");

    let mut temp_partitions: Vec<Partition> = Vec::new();

    for partition in partitions.as_ref().unwrap()
    {
        let path = format_unallocated(
            partitions.as_ref().unwrap(),
            disk_path.as_ref().unwrap(),
            partition.start,
            partition.end,
            partition.filesystem.as_ref().unwrap(),
            partition.label.clone()
        )?;

        temp_partitions.push(
            Partition {
                path,
                ..partition.to_owned()
            }
        )
    }

    println!("partitioning done");

    println!("{:#?}", temp_partitions);

    actual_partitioning(Some(temp_partitions))?;

    Ok(())
}

fn partitioning_layout_changed(blueprint: &BluePrint) -> Result<(), Error>
{
    todo!();
}

fn partitioning_layout_preserved(blueprint: &BluePrint) -> Result<(), Error>
{
    force_umount();

    let firmware_type = &blueprint.bootloader.as_ref().unwrap().firmware_type;
    let bootloader_path = &blueprint.bootloader.as_ref().unwrap().path;

    let partitions = blueprint.storage.as_ref().unwrap().partitions.as_ref().unwrap();

    actual_partitioning(Some(partitions.to_vec()))?;

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
    let disk = &blueprint.storage.as_ref().unwrap().partitions;
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
    let disk = &blueprint.storage.as_ref().unwrap().partitions;
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

fn actual_partitioning(partitions: Option<Vec<Partition>>) -> Result<(), Error>
{
    match partitions
    {
        Some(partitions) => {

            // Find partition that assigned as root, and deal with it first

            let root = partitions.iter().find(|p| p.mountpoint.as_ref().is_some_and(|m| m.eq("/")));

            println!("Checking if root is available");

            if let Some(root) = root
            {
                let mut path: Option<String> = root.to_owned().path;

                println!("Checking if the partition is unallocated");
                if root.format &&
                    root.path.is_none() &&
                    root.filesystem.is_some()
                {
                    println!("Unallocated detected! Formatting unallocated partition");
                    path = format_unallocated(
                        &partitions,
                        root.disk_path.as_ref().unwrap(),
                        root.start,
                        root.end,
                        root.filesystem.as_ref().unwrap(),
                        root.label.clone()
                    )?;
                }

                else if root.format &&
                    root.path.is_some() &&
                    root.filesystem.is_some()
                {
                    println!("Checking the rest of the allocated partition");
                    format(
                        root.filesystem.as_ref().unwrap(),
                        root.path.as_ref().unwrap()
                    )?;
                    println!("Allocated partition formatted");
                }

                if let Some(filesystem) = root.to_owned().filesystem
                {
                    mount(path.as_ref().unwrap(), "/tealinux-mount", None)?;

                    if filesystem.eq("btrfs")
                    {
                        create_subvolume("/tealinux-mount/@")?;
                        create_subvolume("/tealinux-mount/@home")?;

                        umount("/tealinux-mount")?;

                        mount_subvolume("@", path.as_ref().unwrap(), "/tealinux-mount")?;

                        create_dir("/tealinux-mount/home")?;

                        mount_subvolume("@home", path.as_ref().unwrap(), "/tealinux-mount/home")?;
                    }
                }
            }

            // Deal with rest of the partition

            for i in partitions
            {
                if let Some(mountpoint) = i.to_owned().mountpoint
                {
                    // Ignore root

                    if !mountpoint.eq("/")
                    {
                        if mountpoint.contains("swap")
                        {
                            cmd!("swapon", i.path.as_ref().unwrap()).run()?;
                        }

                        if mountpoint.contains("boot")
                        {
                            cmd!("mkdir", "--parents", format!("/tealinux-mount{}", mountpoint)).run()?;
                            mount(i.path.as_ref().unwrap(), &format!("/tealinux-mount{}", mountpoint), None)?;
                        }
                    }
                }
            }

            Ok(())
        },
        None => Err(Error::from(ErrorKind::NotFound))
    }
}
