use std::io::Error;
use duct::cmd;
use std::fs::File;
use std::io::BufReader;
use super::installer::BluePrint;

pub mod filesystem;
pub mod btrfs;

pub fn format(filesystem: &str, path: &str) -> Result<(), Error>
{
    let command = {

        if filesystem == "btrfs"
        {
            format!("mkfs.btrfs --force {}", path)
        }
        else if filesystem == "exfat"
        {
            format!("mkfs.{} {}", filesystem, path)
        }
        else if filesystem.contains("fat")
        {
            let fat = filesystem.trim_matches(|c| c == 'f' || c == 'a' || c == 't');

            format!("mkfs.fat -F {} {}", fat, path)
        }
        else if filesystem.contains("ext")
        {
            format!("mkfs.{} -F {}", filesystem, path)
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

        for i in json.disk.as_ref().unwrap().iter()
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

    for partition in json.disk.as_ref().unwrap().iter()
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
