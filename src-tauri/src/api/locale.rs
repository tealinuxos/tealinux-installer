use tea_arch_chroot_lib::chroot::Locale;
use tea_arch_chroot_lib::resource::Locales;
use std::fs::File;
use std::io::{ BufReader, BufWriter, Read, Write };
use std::path::Path;
use crate::installer::BluePrint;

#[tauri::command]
pub async fn get_locale_json() -> String
{
    Locales::list_json()
}

#[tauri::command]
pub async fn blueprint_set_locale(locale: String)
{
    let locale = Locale::new(&locale);

    let mut blueprint = super::get_blueprint().unwrap();

    blueprint.locale = Some(locale);

    super::write_blueprint(blueprint).unwrap();
}
