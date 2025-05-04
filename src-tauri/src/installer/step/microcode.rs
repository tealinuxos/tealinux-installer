use std::io::Error;
use std::fs;
use serde_json::Value;

use crate::api::get_read_from_opt;

pub async fn copy_microcode() -> Result<(), Error>
{
    let json = get_read_from_opt().await;
    let json: Value = serde_json::from_str(&json)?;

    let microcode = &json["lspci"]["cpu"];

    if let Some(cpu) = microcode.as_str()
    {
        if cpu.to_lowercase().contains("intel")
        {
            let path = "/run/archiso/bootmnt/arch/boot/intel-ucode.img";

            fs::copy(path, "/tealinux-mount/boot/intel-ucode.img")?;
        }

        else if cpu.to_lowercase().contains("amd")
        {
            let path = "/run/archiso/bootmnt/arch/boot/amd-ucode.img";

            fs::copy(path, "/tealinux-mount/boot/amd-ucode.img")?;
        }
    }

    Ok(())
}
