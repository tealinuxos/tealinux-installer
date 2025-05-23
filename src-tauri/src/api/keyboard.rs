use tea_arch_chroot_lib::resource::Keyboard;
use tea_arch_chroot_lib::chroot::keyboard;
use crate::installer;

#[tauri::command]
pub async fn get_keyboard_json() -> String
{
    let keyboard = Keyboard::list();

    serde_json::to_string_pretty(&keyboard).unwrap()
}

#[tauri::command]
pub async fn blueprint_set_keyboard(layout: String, variant: Option<String>)
{
    let keyboard = installer::Keyboard { layout: Some(layout), variant };

    let mut blueprint = super::get_blueprint().unwrap();

    blueprint.keyboard = Some(keyboard);

    super::write_blueprint(blueprint).unwrap();
}

#[tauri::command]
pub async fn set_cosmic_keymap(live: bool, layout: String, variant: Option<String>)
{
    let keymap = keyboard::Keyboard::new(layout, variant);
    let username = String::from("liveuser");

    keymap.set_keymap_cosmic(live, username).unwrap();
}
