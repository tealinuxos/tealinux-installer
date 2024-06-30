use tea_arch_chroot_lib::resource::Timezones;
use tea_arch_chroot_lib::chroot::Timezone;
use crate::installer::BluePrint;

#[tauri::command]
pub async fn get_timezone_json() -> String
{
    Timezones::list_json()
}

#[tauri::command]
pub async fn blueprint_set_timezone(region: String, city: String)
{
    let timezone = Timezone::new(&region, &city);

    let mut blueprint = super::get_blueprint().unwrap();

    blueprint.timezone = Some(timezone);

    super::write_blueprint(blueprint).unwrap();
}
