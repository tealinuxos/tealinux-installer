// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod installer;
mod read;
mod storage;
mod system;
mod utils;

use storage::umount_all_target;
use tauri::webview::WebviewWindowBuilder;
use tauri::AppHandle;
use tauri::RunEvent;
use tauri::WebviewUrl;
use specta_typescript::BigIntExportBehavior;
use specta_typescript::Typescript;
use users::get_current_uid;

fn get_specta_builder() -> tauri_specta::Builder<tauri::Wry> {
    tauri_specta::Builder::<tauri::Wry>::new()
        .commands(tauri_specta::collect_commands![
            api::get_read_json,
            api::set_read_json,
            api::set_empty_blueprint,
            api::get_filesystem_json,
            api::get_read_from_opt,
            api::get_blueprint_from_opt,
            api::read_blueprint,
            api::get_other_os_json,
            api::locale::blueprint_set_locale,
            api::locale::get_locale_json,
            api::timezone::blueprint_set_timezone,
            api::timezone::get_timezone_json,
            api::account::blueprint_set_account,
            api::firmware::blueprint_set_bootloader,
            api::keyboard::blueprint_set_keyboard,
            api::keyboard::get_keyboard_json,
            api::keyboard::set_cosmic_keymap,
            api::partition::get_disk_lists_key_val,
            api::partition::get_disk_lists_key_val_with_otheros_check,
            api::auto_partition::autogen_partition_select_disk,
            api::storage::blueprint_set_storage,
            api::storage::read_refresh_disk,
            /*
             * from ./installer/
             */
            installer::start_install,
            installer::is_online,
            installer::print_json,
            installer::step::partitioning::test_partitioning,
            /*
             * from ./system/
             */
            system::reboot::reboot,
            system::spawn::spawn_gparted,
            system::spawn::spawn_terminal,
            open_website,
        ])
        .typ::<installer::BluePrint>()
}

fn main() {
    match get_current_uid() {
        0 => {
            build_tauri();
        }
        _ => {
            karen::pkexec_with_env(&[
                "WAYLAND_DISPLAY",
                "XDG_RUNTIME_DIR",
                "XDG_SESSION_TYPE",
                "XDG_CURRENT_DESKTOP",
            ])
            .unwrap();

            duct::cmd!("xhost", "si:localuser:root")
                .run()
                .expect("Failed to run xhost, does it exist?");

            build_tauri();
        }
    }
}

fn build_tauri() {
    let tauri_specta_builder = get_specta_builder();

    #[cfg(debug_assertions)]
    {
        let mut specta_tsconfig: Typescript = Typescript::default();
        specta_tsconfig.bigint = BigIntExportBehavior::BigInt;

        let _ = tauri_specta_builder.export(specta_tsconfig, "../src/bindings.ts");
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri_specta_builder.invoke_handler())
        .setup(move |app| {
            // This is also required if you want to use events
            tauri_specta_builder.mount_events(app);

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, _event| {
            if let RunEvent::Exit = _event {
                let _ = duct::cmd!("xhost", "-si:localuser:root").run();
                let _ = umount_all_target("/tealinux-mount");
            }
        });
}

#[tauri::command]
#[specta::specta]
fn open_website(app: AppHandle) {
    let url = WebviewUrl::External("https://tealinuxos.org".parse().unwrap());

    WebviewWindowBuilder::new(&app, "webview", url)
        .title("TeaLinuxOS")
        .build()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use specta_typescript::BigIntExportBehavior;
    use specta_typescript::Typescript;

    #[test]
    fn export_bindings() {
        let mut specta_tsconfig: Typescript = Typescript::default();
        specta_tsconfig.bigint = BigIntExportBehavior::BigInt;

        get_specta_builder()
            .export(specta_tsconfig, "../src/bindings.ts")
            .expect("Failed to export typescript bindings");

        println!("Bindings generated successfully at ../src/bindings.ts");
    }
}
