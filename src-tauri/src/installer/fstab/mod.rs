use super::commands::*;

#[tauri::command]
pub async fn generate_fstab() -> bool
{
    let cmd = format!("genfstab -U /mnt > /mnt/etc/fstab");

    let status = command_with_status(cmd);

    status
}
