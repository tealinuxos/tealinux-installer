fn main() {
    match std::env::var("TEALINUX_BUILD") {
        Ok(val) => {
            println!("cargo:warning=TEALINUX_BUILD is set to: {}", val);
            tauri_build::build()
        }
        Err(_) => {
            // If not set, set to dev
            println!("cargo:warning=Environment variable TEALINUX_BUILD is not set during build. Automatically set to dev. See https://github.com/tealinuxos/tealinux-installer?tab=readme-ov-file#building");
            unsafe 
            {
                std::env::set_var("TEALINUX_BUILD", "dev");
            }
            if let Ok(val) = std::env::var("TEALINUX_BUILD")
            {
                println!("cargo:warning=TEALINUX_BUILD is set to: {}", val);
                tauri_build::build()
            }
            else
            {
                eprintln!("error: Environment variable TEALINUX_BUILD is not set during build. Aborting build. see https://github.com/tealinuxos/tealinux-installer?tab=readme-ov-file#building");
                std::process::exit(1);
            }
        }
    }
}
