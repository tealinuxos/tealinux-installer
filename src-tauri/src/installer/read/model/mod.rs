#[tauri::command]
pub fn model_number() -> String
{
    String::from("T480")
}

#[tauri::command]
pub fn brand() -> String
{
    String::from("ThinkPad")
}
