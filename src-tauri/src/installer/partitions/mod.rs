use sysinfo::Disks;
use super::commands::*;

pub async fn format_fat32(partition: String) -> bool
{
    let command = format!("mkfs.fat -F32 {}", partition);
    
    command_with_status(command)
}

pub async fn format_ext4(partition: String) -> bool
{
    let command = format!("mkfs.ext4 --force {}", partition);

    command_with_status(command)
}

pub async fn format_btrfs(partition: String) -> bool
{
    let command = format!("mkfs.btrfs --force {}", partition);

    command_with_status(command)
}

pub async fn mount_as(partition: String, mount_point: String) -> bool
{
    let command = format!("mount {} {}", partition, mount_point);

    command_with_status(command)
}

#[tauri::command]
pub async fn get_disk_info() -> Vec<(String, String, u64, u64, String)>
{
    let mut vec: Vec<(String, String, u64, u64, String)> = Vec::new();
    let mut disks = Disks::new_with_refreshed_list();

    disks.sort_by_key(|i| i.name().to_string_lossy().to_mut().clone());

    for disk in disks.list()
    {
        let par = (disk.name().to_str().unwrap().to_string(), disk.file_system().to_str().unwrap().to_string(), disk.available_space(), disk.total_space(), disk.mount_point().to_str().unwrap().to_string());
        vec.push(par);
    }
    
    vec
}
