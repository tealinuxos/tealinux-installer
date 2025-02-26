use crate::error;
use duct::cmd;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
// this file is used to calculate how much partition should be

#[derive(Serialize, Deserialize, Debug)]
pub struct Partitions {
    node: String,
    start: u64,
    size: u64,
    #[serde(rename = "type")]
    _type: String,
    #[serde(default)]
    bootable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlkiIitialData {
    label: String,
    id: String,
    device: String,
    unit: String,
    sectorsize: u64,
    partitions: Vec<Partitions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartitionTable {
    partitiontable: BlkiIitialData,
}

#[derive(Debug)]
pub struct Blkstuff {
    selected: String,
    partitiontable: PartitionTable,
}

//  this file is derived from src-tauri/src/installer/blueprint.rs:18
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Partition {
    pub number: u64,
    pub disk_path: Option<String>,
    pub path: Option<String>,
    pub mountpoint: Option<String>,
    pub filesystem: Option<String>,
    pub format: bool,
    pub start: u64,
    pub end: u64,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlkCalcResult {
    pub number: i32,
    #[serde(rename = "diskPath")]
    pub disk_path: String,
    pub path: Option<String>,
    pub mountpoint: Option<String>,
    pub filesystem: Option<String>,
    pub format: bool,
    pub start: u64,
    pub end: u64,
    pub size: u64,
}

pub trait Blkutils {
    fn blockdevice(blkname: String) -> Self;
    fn get_blkinfo(blkname: &String) -> Result<PartitionTable, String>;
    fn getblkbytes(&self) -> Option<u64>;
}

impl Blkutils for Blkstuff {
    fn blockdevice(blkname: String) -> Self {
        let _blkdata = Self::get_blkinfo(&blkname).unwrap_or_else(|e| {
            eprintln!("ERROR!!!!!!: {}", e);
            PartitionTable {
                partitiontable: BlkiIitialData {
                    label: "".to_string(),
                    id: "".to_string(),
                    device: "".to_string(),
                    unit: "".to_string(),
                    sectorsize: 0,
                    partitions: Vec::new(),
                },
            }
        });

        Blkstuff {
            selected: blkname,
            partitiontable: _blkdata,
        }
    }

    fn get_blkinfo(blkname: &String) -> Result<PartitionTable, String> {
        let sfdisk_res = cmd!("sfdisk", "--json", blkname).read();

        if let Ok(sfdisk_res_val) = sfdisk_res {
            let sfdisk_parsed: Result<PartitionTable, serde_json::Error> =
                serde_json::from_str::<PartitionTable>(&sfdisk_res_val);
            // self.blkdata = sfdisk_parsed;

            match sfdisk_parsed {
                Ok(val) => Ok(val),
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err("sfdisk error!!!!".to_string())
        }
    }

    /// this func return how many bytes of data
    fn getblkbytes(&self) -> Option<u64> {
        let data = cmd!("blockdev", "--getsize64", self.selected.clone()).read();
        // println!("{:#?}", data);

        if let Ok(data_val) = data {
            let ret = u64::from_str(&data_val).unwrap();
            // println!("convert {:#?}", dat);

            Some(ret)
        } else {
            None
        }
    }

    // fn convert_block2bytes()
}
