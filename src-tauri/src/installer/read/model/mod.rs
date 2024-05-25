#[tauri::command]
pub fn get_model_number() -> String
{
    String::from("T480")
}

#[tauri::command]
pub fn get_brand() -> String
{
    String::from("ThinkPad")
}
