use crate::installer::blueprint::OriginalSector;
use crate::installer::blueprint::Partition;

use duct::cmd;
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use std::{clone, str::FromStr};
use tea_partition_generator::core::ListsAllSpace;
use tea_partition_generator::single_boot_blockdev::{Blkstuff, SingleBootBlockdevice};

use tea_partition_generator::core::{PartitionGenerator, TeaPartitionGenerator};
use tea_partition_generator::dual_boot_blockdev::{DualBootBlockdevice, DualbootBlkstuff};

#[tauri::command]
pub async fn autogen_partition_select_disk(
    blkname: String,
    mode: String,
    partition_table: String,
    fs: String,
    use_swap: bool,
    start: Option<u64>,
    end: Option<u64>,
) -> Result<(), String> {
    // Handle singleboot
    if mode == "singleboot" {
        println!(
            "generating config {} for {} with fs {} -> {}, swap: {}",
            mode, blkname, fs, partition_table, use_swap
        );
        let ctx: Blkstuff =
            SingleBootBlockdevice::blockdevice(blkname, fs, partition_table, use_swap);

        let ret = ctx.getresult();

        match ret {
            Ok(ret_val) => {
                let mut blueprint = super::get_blueprint().unwrap();

                blueprint.storage = Some(ret_val.into());

                let bootloader = ctx.gen_current_bootloader();
                if let Some(bootloader_val) = bootloader {
                    blueprint.bootloader = Some(bootloader_val.into());
                } else {
                    let errstr = "failed to generate bootloader for autopartitioning.".to_string();
                    println!("{}", errstr.clone());
                    return Err(errstr.clone());
                }

                // this is truely unused, for formality
                blueprint.storage.clone().unwrap().original_sector = Option::None;

                super::write_blueprint(blueprint).unwrap();
                return Ok(());
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }
        // if let Ok(ret_val) = ret {
        //     // Start werite json
        // }
    } else if mode == "dualboot" {
        println!(
            "generating config dualboot {} with fs {} swap: {}",
            blkname, fs, use_swap
        );

        let mut ctx: DualbootBlkstuff =
            DualBootBlockdevice::blockdevice(blkname.clone(), fs, use_swap);

        // let partition_generator_ctx = TeaPartitionGenerator::new(blkname.clone());
        // let (start, end) = partition_generator_ctx.find_empty_space_sector_area();

        // should be guarantee not none (from frontend)
        println!("{:?}", start);
        if start.unwrap() == 0 && end.unwrap() == 0 {
            // TODO: Add
            let errstr = "no empty partition, aborting!".to_string();
            println!("{}", errstr.clone());

            return Err(errstr.clone());
        }

        let ret = ctx.getresult(start.unwrap(), end.unwrap());

        match ret {
            Ok(ret_val) => {
                let mut blueprint = super::get_blueprint().unwrap();

                blueprint.storage = Some(ret_val.into());

                let bootloader = ctx.gen_current_bootloader();
                if let Some(bootloader_val) = bootloader {
                    blueprint.bootloader = Some(bootloader_val.into());
                } else {
                    println!("failed to generate bootloader for autopartitioning.");
                }

                // now is none, but used later in libs internal
                blueprint.storage.clone().unwrap().original_sector = Option::None;

                super::write_blueprint(blueprint).unwrap();
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        }

        // if let Ok(ret_val) = ret {
        //     // Start werite json
        // }
    } else {
        // mode manual, ignored
        return Ok(());
    }

    // You can add more functionality as needed
}

#[tauri::command]
pub async fn get_partition_sector_lists(selected_disks: String) -> Vec<ListsAllSpace> {
    let ctx = TeaPartitionGenerator::new(selected_disks);
    return ctx.find_partition_sector_areav();
}
