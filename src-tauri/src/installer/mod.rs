pub mod blueprint;

use std::fs::File;
use std::io::BufReader;
use tea_arch_chroot_lib::prechroot::*;

pub use self::blueprint::BluePrint;

#[tauri::command]
pub async fn start_install()
{
    let file = File::open("/opt/installer.json").unwrap();
    let reader = BufReader::new(file);

    let json: BluePrint = serde_json::from_reader(reader).unwrap();

    println!("{:#?}", json);
}
