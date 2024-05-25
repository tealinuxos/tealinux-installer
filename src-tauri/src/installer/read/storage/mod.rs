#[tauri::command]
pub fn get_storage_capacity() -> u64
{
    512000
}

#[tauri::command]
pub fn get_partitions() -> Vec<String>
{
    vec![String::from("/dev/sda1"), String::from("/dev/sda2"), String::from("/dev/sda3")]
}

#[tauri::command]
pub fn get_flags(partition: String) -> Vec<String>
{
    vec![String::from("esp"), String::from("msftdata")]
}

#[tauri::command]
pub fn get_mountpoints(partition: String) -> Vec<String>
{
    vec![String::from("/")]
}

#[tauri::command]
pub fn get_filesystem(partition: String) -> String
{
    String::from("BTRFS")
}
