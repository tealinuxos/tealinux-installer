use tea_arch_chroot_lib::resource::Keyboard;

#[tauri::command]
pub async fn get_keyboard_json() -> String
{
    let keyboard = Keyboard::list();

    serde_json::to_string_pretty(&keyboard).unwrap()
}
