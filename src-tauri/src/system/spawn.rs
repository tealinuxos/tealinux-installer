use duct::cmd;

#[tauri::command]
pub fn spawn_gparted()
{
    cmd!("gparted").run().unwrap();
}

#[tauri::command]
pub fn spawn_terminal()
{
    cmd!("kgx").run().unwrap();
}
