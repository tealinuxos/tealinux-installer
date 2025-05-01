use crate::installer::blueprint::Partition;
use duct::cmd;
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use std::{clone, str::FromStr};
use tea_partition_generator::single_boot_blockdev::{Blkstuff, SingleBootBlockdevice};

#[tauri::command]
pub async fn autogen_partition_select_disk(
    blkname: String,
    mode: String,
    partition_table: String,
    fs: String,
) -> () {
    // Handle singleboot
    if mode == "singleboot" {
        println!(
            "generating config {} for {} with fs {} -> {}",
            mode, blkname, fs, partition_table
        );
        let ctx: Blkstuff = SingleBootBlockdevice::blockdevice(blkname, fs, partition_table);

        let ret = ctx.getresult();

        if let Ok(ret_val) = ret {
            // Start werite json
            let mut blueprint = super::get_blueprint().unwrap();

            blueprint.storage = Some(ret_val.into());

            let bootloader = ctx.gen_current_bootloader();
            if let Some(bootloader_val) = bootloader {
                blueprint.bootloader = Some(bootloader_val.into());
            } else {
                println!("failed to generate bootloader for autopartitioning.");
            }

            super::write_blueprint(blueprint).unwrap();
        }
    }

    // You can add more functionality as needed
}
