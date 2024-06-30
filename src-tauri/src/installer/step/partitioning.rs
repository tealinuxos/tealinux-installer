use crate::installer::BluePrint;
use crate::storage::{ format, mount, umount };
use crate::storage::btrfs::{ mount_subvolume, create_subvolume };
use duct::cmd;
use std::io::Error;
use std::fs::create_dir;

pub fn partitioning(blueprint: &BluePrint) -> Result<(), Error>
{
    for i in blueprint.disk.as_ref().unwrap().iter().to_owned().rev()
    {
        let i_path = &i.path;
        let i_format = &i.format;
        let i_mountpoint = &i.mountpoint;

        if let Some(i_format) = i_format
        {
            format(i_format.as_ref(), i_path.as_ref().unwrap())?;
        }

        if let Some(i_mountpoint) = i_mountpoint
        {
            if i_mountpoint.contains("boot")
            {
                cmd!("mkdir", "--parents", format!("/mnt{}", i_mountpoint)).run().unwrap();
                mount(i_path.as_ref().unwrap(), &format!("/mnt{}", i_mountpoint), None)?;
            }

            else if i_mountpoint.eq("/") && i_format.as_ref().unwrap().eq("btrfs")
            {
                mount(i.path.as_ref().unwrap(), "/mnt", None)?;

                create_subvolume("/mnt/@")?;
                create_subvolume("/mnt/@home")?;

                umount("/mnt")?;

                mount_subvolume("@", i.path.as_ref().unwrap(), "/mnt")?;

                create_dir("/mnt/home")?;

                mount_subvolume("@home", i.path.as_ref().unwrap(), "/mnt/home")?;
            }

            else
            {
                mount(i_path.as_ref().unwrap(), &format!("/mnt{}", i_mountpoint), None)?;
            }
        }
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
