use duct::cmd;
use std::fs::{ rename, copy };
use std::io::{Error, ErrorKind};
use std::path::Path;
use uname::uname;

pub fn copy_kernel() -> Result<u64, Error>
{
    let kernel_version = uname().unwrap().release;

    let kernel_path = format!("/run/archiso/airootfs/usr/lib/modules/{}/vmlinuz", kernel_version);

    let kernel_path = Path::new(&kernel_path);
    let boot_path = Path::new("/mnt/boot/vmlinuz-linux");

    let mut size = 0;

    if kernel_path.exists()
    {
        size = copy(kernel_path, boot_path)?;
    }
    else
    {
        Err(ErrorKind::NotFound)?;
    }

    Ok(size)
}

pub fn copy_mkinitcpio_preset() -> Result<u64, Error>
{
    let preset_path_origin = Path::new("/etc/mkinitcpio.d/linux-deploy.preset");
    let preset_path_destination = Path::new("/mnt/etc/mkinitcpio.d/linux.preset");

    let mut size = 0;

    if preset_path_origin.exists()
    {
        size = copy(preset_path_origin, preset_path_destination)?;
    }
    else
    {
        Err(ErrorKind::NotFound)?;
    }

    Ok(size)
}
