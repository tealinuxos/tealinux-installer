#[tauri::command]
pub fn get_memory_capacity() -> u32
{
    8192
}

#[tauri::command]
pub fn get_memory_free() -> u32
{
    6144
}
