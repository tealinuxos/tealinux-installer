use duct::cmd;

#[tauri::command]
#[specta::specta]
pub fn reboot()
{
    cmd!("systemctl", "reboot").run().expect("Failed to reboot system");
}
