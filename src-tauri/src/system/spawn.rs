use duct::cmd;

#[tauri::command]
#[specta::specta]
pub fn spawn_gparted()
{
    cmd!("gparted").run().unwrap();
}

#[tauri::command]
#[specta::specta]
pub fn spawn_terminal()
{
    cmd!("kgx").run().unwrap();
}
