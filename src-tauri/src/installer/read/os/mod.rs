#[tauri::command]
pub fn get_other_os() -> Vec<String>
{
    vec![String::from("Windows 10")]
}
