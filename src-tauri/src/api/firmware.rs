use crate::installer::Bootloader;

#[tauri::command]
pub fn blueprint_set_bootloader(bootloader: String)
{
    let bootloader: Bootloader = serde_json::from_str(&bootloader).unwrap();

    let mut blueprint = super::get_blueprint().unwrap();

    blueprint.bootloader = Some(bootloader);

    super::write_blueprint(blueprint).unwrap();
}
