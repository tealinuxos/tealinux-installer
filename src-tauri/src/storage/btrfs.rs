use duct::cmd;
use std::io::Error;
use super::mount;

pub fn create_subvolume(path: &str) -> Result<(), Error>
{
    cmd!("btrfs", "subvolume", "create", path).run()?;

    Ok(())
}

pub fn mount_subvolume(subvol: &str, path: &str, mountpoint: &str) -> Result<(), Error>
{
    let subvol = format!("subvol={}", subvol);

    let options = vec![
        &subvol,
        "compress=zstd"
    ];

    mount(path, mountpoint, Some(options))?;

    Ok(())
}
