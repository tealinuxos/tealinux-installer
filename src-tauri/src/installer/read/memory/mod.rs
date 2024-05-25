#[tauri::command]
pub fn memory_capacity() -> u32
{
    8192
}

#[tauri::command]
pub fn memory_free() -> u32
{
    6144
}
