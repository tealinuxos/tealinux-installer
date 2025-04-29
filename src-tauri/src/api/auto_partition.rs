use crate::installer::blueprint::Partition;
use duct::cmd;
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use std::{clone, str::FromStr};
use tea_partition_generator::single_boot_blockdev::{Blkstuff, SingleBootBlockdevice};

impl From<tea_partition_generator::blueprint::Partition> for crate::installer::Partition {
    fn from(data: tea_partition_generator::blueprint::Partition) -> Self {
        crate::installer::Partition {
            number: data.number,
            disk_path: data.disk_path,
            path: data.path,
            mountpoint: data.mountpoint,
            filesystem: data.filesystem,
            format: data.format,
            start: data.start,
            end: data.end,
            size: data.size,
        }
    }
}

fn convert_partition(
    opt: Option<Vec<tea_partition_generator::blueprint::Partition>>,
) -> Option<Vec<crate::installer::Partition>> {
    opt.map(|vec| vec.into_iter().map(Into::into).collect())
}

impl From<tea_partition_generator::blueprint::Storage> for crate::installer::Storage {
    fn from(data: tea_partition_generator::blueprint::Storage) -> Self {
        crate::installer::Storage {
            disk_path: data.disk_path,
            partition_table: data.partition_table,
            new_partition_table: data.new_partition_table,
            layout_changed: data.layout_changed,
            partitions: convert_partition(data.partitions), 
        }
    }
}

#[tauri::command]
pub async fn autogen_partition_select_disk(
    blkname: String,
    mode: String,
    partition_table: String,
    fs: String,
) -> () {
    // Handle singleboot
    if mode == "singleboot" {
        let ctx: Blkstuff = SingleBootBlockdevice::blockdevice(
            "/dev/sdb".to_string(),
            "btrfs".to_string(),
            "gpt".to_string(),
        );

        let ret = ctx.getresult();

        if let Ok(ret_val) = ret {
            // Start werite json
            let mut blueprint = super::get_blueprint().unwrap();

            blueprint.storage = Some(ret_val.into()); 

            super::write_blueprint(blueprint).unwrap();
        }
    }

    // You can add more functionality as needed
}
