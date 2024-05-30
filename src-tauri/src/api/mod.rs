use crate::read::get_read;

#[tauri::command]
pub async fn get_read_json() -> String
{
    let read = get_read();

    let json = serde_json::to_string_pretty(&read);

    json.unwrap()
}
