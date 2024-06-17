pub mod blueprint;

use std::fs::File;
use std::io::BufReader;
use tea_arch_chroot_lib::prechroot::*;
use tea_arch_chroot_lib::chroot::*;
use super::read::online::Online;

pub use self::blueprint::BluePrint;

#[tauri::command]
pub async fn start_install()
{
    let file = File::open("/opt/installer.json").unwrap();
    let reader = BufReader::new(file);

    let json: BluePrint = serde_json::from_reader(reader).unwrap();

    println!("{:#?}", json);

    // PreChroot

    // rsync::start_rsync().await.unwrap();
    // fstab::generate_fstab().await.unwrap();
    //
    //
    // // Chroot
    // 
    // pacman::regenerate_pacman_key().unwrap();
    // pacman::install_package(vec!["mkinitcpio", "neovim"]).unwrap();
    // // pacman::update();
    //
    // json.locale.set_locale().unwrap();
    // json.timezone.generate_localtime().unwrap();
    // json.account.set_host().unwrap();
    // json.account.add_user().unwrap();
}

#[tauri::command]
pub async fn is_online() -> bool
{
    let online = Online::new();

    online.status()
}
