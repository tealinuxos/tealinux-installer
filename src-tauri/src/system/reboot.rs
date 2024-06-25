use duct::cmd;

#[tauri::command]
pub fn reboot()
{
    cmd!("systemctl", "reboot").run().expect("Failed to reboot system");
}
