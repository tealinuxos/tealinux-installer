use tea_arch_chroot_lib::resource::Keyboard;
use crate::installer;

#[tauri::command]
pub async fn get_keyboard_json() -> String
{
    let keyboard = Keyboard::list();

    serde_json::to_string_pretty(&keyboard).unwrap()
}

#[tauri::command]
pub async fn blueprint_set_keyboard(layout: String, variant: String)
{
    let keyboard = installer::Keyboard { layout: Some(layout), variant: Some(variant) };

    let mut blueprint = super::get_blueprint().unwrap();

    blueprint.keyboard = Some(keyboard);

    super::write_blueprint(blueprint).unwrap();
}
