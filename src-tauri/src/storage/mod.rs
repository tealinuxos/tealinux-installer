use tea_partition_api_lib::{ Disk, Partition };
use tea_partition_api_lib::read::get_partition;
use std::io::Error;
use duct::cmd;

pub mod filesystem;

pub async fn get_storage() -> Vec<Disk>
{
    get_partition::parted_list_partition()
}

pub async fn format(filesystem: String, path: String) -> Result<(), Error>
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
        else
        {
            format!("mkfs.{} {}", filesystem, path)
        }
    };

    let command: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();

    cmd(&command[0], &command[1..]).run()?;

    Ok(())
}

pub async fn mount(partition_path: String, mountpoint: String, options: Option<Vec<String>>) -> Result<(), Error>
{
    let options = {
        if options.is_some()
        {
            let options: String = options
                .unwrap()
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

pub fn umount_all() -> Result<(), Error>
{
    cmd!("umount", "--all").run()?;

    Ok(())
}
