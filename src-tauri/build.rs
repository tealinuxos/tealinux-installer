fn main() {
    match std::env::var("TEALINUX_BUILD") {
        Ok(val) => {
            println!("cargo:warning=TEALINUX_BUILD is set to: {}", val);
            tauri_build::build()
        }
        Err(_) => {
            // Abort build if not set
            eprintln!("error: Environment variable TEALINUX_BUILD is not set during build. Aborting build. see https://github.com/tealinuxos/tealinux-installer?tab=readme-ov-file#building");
            std::process::exit(1);
        }
    }
}
