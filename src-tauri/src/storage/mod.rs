use std::io::Error;
use std::process::Output;
use duct::cmd;
use std::fs::File;
use std::io::BufReader;
use crate::installer::Partition;
use crate::read::get_read;

use super::installer::BluePrint;

pub mod filesystem;
pub mod btrfs;

pub fn wipe(path: &str) -> Result<(), Error>
{
    let _wipefs = cmd!("wipefs", "--all", path).run()?;

    Ok(())
}

pub fn format(filesystem: &str, path: &str, label: Option<&str>) -> Result<(), Error>
{
    let command = {

        if filesystem == "btrfs"
        {
            let label = label.as_ref().map(|l| format!("--label {}", l)).unwrap_or_default();
            format!("mkfs.btrfs --force {} {}", label, path)
        }
        else if filesystem == "exfat"
        {
            let label = label.as_ref().map(|l| format!("-n {}", l)).unwrap_or_default();
            format!("mkfs.exfat {} {}", label, path)
        }
        else if filesystem.contains("fat")
        {
            let fat = filesystem.trim_matches(|c| c == 'f' || c == 'a' || c == 't');

            let label = label.as_ref().map(|l| format!("-n {}", l)).unwrap_or_default();
            format!("mkfs.fat -F {} {} {}", fat, label, path)
        }
        else if filesystem.contains("ext")
        {
            let label = label.as_ref().map(|l| format!("-L {}", l)).unwrap_or_default();
            format!("mkfs.{} {} -F {}", filesystem, label, path)
        }
        else if filesystem.contains("swap")
        {
            format!("mkswap --force --quiet {}", path)
        }
        else
        {
            format!("mkfs.{} {}", filesystem, path)
        }
    };

    let command: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();

    cmd(&command[0], &command[1..]).run()?;

    Ok(())
}

pub fn mount(partition_path: &str, mountpoint: &str, options: Option<Vec<&str>>) -> Result<(), Error>
{
    let options = {
        if let Some(options) = options
        {
            let options: String = options
                .iter()
                .map(|o| format!("{},", o))
                .collect();

            Some(options)
        }
        else
        {
            None
        }
    };

    if options.is_none()
    {
        cmd!("mount", partition_path, mountpoint).run()?;
    }
    else
    {
        cmd!("mount", "-o", options.unwrap(), partition_path, mountpoint).run()?;
    }

    Ok(())
}

pub fn umount(path: &str) -> Result<(), Error>
{
    cmd!("umount", path).run()?;

    Ok(())
}

pub fn umount_all_target(target: &str) -> Result<(), Error>
{
    cmd!("umount", "--all-targets", target, "--recursive").run()?;

    Ok(())
}

pub fn umount_all() -> Result<(), Error>
{
    let file = File::open("/opt/tea-installer/installer.json").unwrap();
    let reader = BufReader::new(file);

    let json: BluePrint = serde_json::from_reader(reader).unwrap();

    let has_root = {

        let mut has_root = false;

        for i in json.storage.as_ref().unwrap().partitions.as_ref().unwrap().iter()
        {
            if let Some(partition) = &i.mountpoint
            {
                if partition.eq("/")
                {
                    has_root = true;
                }
            }
        }

        has_root
    };

    for partition in json.storage.as_ref().unwrap().partitions.as_ref().unwrap().iter()
    {
        let mountpoint = &partition.mountpoint;

        if let Some(mountpoint) = mountpoint
        {
            let mountpoint = format!("/mnt{}", mountpoint);

            if mountpoint != "/mnt/"
            {
                umount(&mountpoint)?;
            }
        }
    }

    if has_root
    {
        umount("/mnt/")?;
    }

    Ok(())
}

pub fn format_unallocated(disk_path: &str, start: u64, end: u64, filesystem: Option<&str>, label: Option<String>) -> Result<Option<String>, Error>
{
    let start = format!("{}s", start);
    let end = format!("{}s", end);

    if let Some(filesystem) = filesystem
    {
        cmd!("parted", "--script", "--fix", disk_path, "mkpart", "primary", filesystem, &start, &end).run()?;
    }
    else
    {
        cmd!("parted", "--script", "--fix", disk_path, "mkpart", "primary", &start, &end).run()?;
    }

    let path = self::get_path_from_sector(disk_path, &start, &end);

    if filesystem.is_none() && path.is_some()
    {
        wipe(path.as_ref().unwrap())?;
    }

    if let Some(path) = path.as_ref()
    {
        if let Some(filesystem) = filesystem
        {
            format(filesystem, path, label.as_deref())?;
        }
    }

    Ok(path)
}

pub fn get_path_from_number(disk_path: &str, number: usize) -> Result<String, Error>
{
    let disk = get_read().disk;

    let mut result = String::new();

    for i in disk
    {
        if let Some(disk) = i.disk_path
        {
            if disk.eq(disk_path)
            {
                if let Some(partition) = i.partitions
                {
                    result  = partition
                        .iter()
                        .filter(|partition| partition.number.is_some() && partition.number.as_ref().unwrap().eq(&number.to_string()))
                        .map(|partition| if partition.partition_path.is_some() { partition.partition_path.as_ref().unwrap() } else { "none" })
                        .collect();
                }
            }
        }
    }

    Ok(result)
}

pub fn get_path_from_sector(disk_path: &str, start: &str, end: &str) -> Option<String>
{
    let disk = get_read().disk;

    let mut result: Option<String> = None;

    for i in disk
    {
        if i.disk_path.is_some_and(|path| path.eq(disk_path))
        {
            if let Some(partitions) = i.partitions
            {
                for j in partitions
                {
                    if j.start.is_some_and(|s| s.as_str() == start) &&
                        j.end.is_some_and(|e| e.as_str() == end)
                    {
                        result = j.partition_path
                    }
                }
            }
        }
    }

    result
}

pub fn create_new_partition_table(disk_path: &str, partition_table: &str) -> Result<Output, Error>
{
    let result = match partition_table
    {
        "gpt" => {
            duct::cmd!("parted", disk_path, "--script", "mklabel", "gpt").run()?
        },
        "uefi" => {
            duct::cmd!("parted", disk_path, "--script", "mklabel", "gpt").run()?
        },
        "mbr" => {
            duct::cmd!("parted", disk_path, "--script", "mklabel", "msdos").run()?
        },
        "bios" => {
            duct::cmd!("parted", disk_path, "--script", "mklabel", "msdos").run()?
        },
        "msdos" => {
            duct::cmd!("parted", disk_path, "--script", "mklabel", "msdos").run()?
        },
        _ => {
            duct::cmd!("parted", disk_path, "--script", "mklabel", "gpt").run()?
        }
    };

    Ok(result)
}

pub fn create_new_partition_table_with_partition(disk_path: &str, partition_table: &str, partitions: Vec<Partition>) -> Result<(), Error>
{
    let parted_partition_table = match partition_table
    {
        "gpt" => "gpt",
        _ => "msdos"
    };

    let args = format!("parted {} mklabel {} --script --fix", disk_path, parted_partition_table);

    duct::cmd!("bash", "-c", &args).run()?;

    match partition_table
    {
        "gpt" => {

            for i in partitions.iter().cloned()
            {
                let label = if let Some(l) = i.label
                {
                    l
                }
                else
                {
                    String::from(r"\'\'")
                };
                    
                let args = format!("parted {} mkpart {} {} {}s {}s --script --fix", disk_path, label, i.filesystem.unwrap(), i.start, i.end);

                println!("{}", args);

                duct::cmd!("bash", "-c", &args).run()?;

            }
        }
        _ => {

            for i in partitions.iter().cloned()
            {
                let args = format!("parted {} mkpart primary {} {}s {}s --script --fix", disk_path, i.filesystem.unwrap(), i.start, i.end);

                println!("{}", args);

                duct::cmd!("bash", "-c", &args).run()?;
            }
        }
    }

    Ok(())
}
