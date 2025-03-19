use crate::utils::error;
use duct::cmd;
use std::{clone, str::FromStr};

// use crate::macros_ab::gb2sector;
use crate::utils::convert::{gb2sector, mb2sector};
use serde::{Deserialize, Serialize};

// bring partition
use crate::installer::blueprint::Partition;
// this file is used to calculate how much partition should be

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Partitions {
    node: String,
    start: u64,
    size: u64,
    #[serde(rename = "type")]
    _type: String,
    #[serde(default)]
    bootable: Option<bool>,

    // this is GPT only
    uuid: Option<String>,
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlkiIitialData {
    pub label: String,
    pub id: String,
    pub device: String,
    pub unit: String,
    pub sectorsize: u64,
    pub partitions: Option<Vec<Partitions>>,

    // this is GPT field
    pub firstlba: Option<u64>,
    pub lastlba: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartitionTable {
    pub partitiontable: BlkiIitialData,
}

#[derive(Debug, Clone)]
pub struct Blkstuff {
    pub selected: String,
    pub partitiontable: PartitionTable,
}

// //  this file is derived from src-tauri/src/installer/blueprint.rs:18
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Partition {
//     pub number: u64,
//     pub disk_path: Option<String>,
//     pub path: Option<String>,
//     pub mountpoint: Option<String>,
//     pub filesystem: Option<String>,
//     pub format: bool,
//     pub start: u64,
//     pub end: u64,
//     pub size: u64,
// }

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
    fn getblksector(&self) -> Option<u64>;
    fn getresult(&self) -> Result<Vec<Partition>, Box<dyn std::error::Error>>;
    fn _export_data(&self) -> ();
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
                    partitions: Some(Vec::new()),
                    firstlba: Some(0),
                    lastlba: Some(0),
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

    /// this func return how many sector of disk
    fn getblksector(&self) -> Option<u64> {
        let data = cmd!("blockdev", "--getsz", self.selected.clone()).read();
        // println!("{:#?}", data);

        if let Ok(data_val) = data {
            let ret = u64::from_str(&data_val).unwrap();
            // println!("convert {:#?}", dat);

            Some(ret)
        } else {
            None
        }
    }

    fn getresult(&self) -> Result<Vec<Partition>, Box<dyn std::error::Error>> {
        // let Ok(blksize) = self.partitiontable.partitiontable.sectorsize;
        let current_size = self.getblkbytes();
        let current_size_sector = self.getblksector();

        // this func itended to return as json
        let mut disks_export: Vec<Partition> = Vec::new();

        let _current_size_val = match current_size {
            Some(size) => size,
            None => {
                return Err(Box::new(error::TealinuxAutoPartitionErr::InternalErr(
                    "something error with getblkbytes()".to_string(),
                )))
            }
        };

        // larger than 200 gb
        if current_size.unwrap() > (200 * 1024 * 1024 * 1024) {
            // setup 512 MB for GPT stuff
            // let mut last_sector: u64 = gb2sector(70, self.partitiontable.partitiontable.sectorsize);
            println!("{:#?}", self.partitiontable);

            disks_export.push(Partition {
                number: 0,
                disk_path: Some(self.selected.clone()),
                path: Some(format!("{}1", self.selected.clone())),
                mountpoint: Some("/boot/efi".to_string()),
                filesystem: Some("fat32".to_string()),
                format: true,
                start: 2048, // aligment
                end: 2048 + mb2sector(512, self.partitiontable.partitiontable.sectorsize),
                size: mb2sector(512, self.partitiontable.partitiontable.sectorsize),
            });

            // align + size (prev)
            let mut last_sector: u64 =
                2048 + mb2sector(512, self.partitiontable.partitiontable.sectorsize);

            // this is root partition
            disks_export.push(Partition {
                number: 1,
                disk_path: Some(self.selected.clone()),
                path: Some(format!("{}2", self.selected.clone())),
                mountpoint: Some("/".to_string()),
                filesystem: Some("ext4".to_string()),
                format: true,
                start: last_sector + 1,
                end: last_sector + gb2sector(70, self.partitiontable.partitiontable.sectorsize),
                size: gb2sector(70, self.partitiontable.partitiontable.sectorsize),
            });

            last_sector =
                last_sector + gb2sector(70, self.partitiontable.partitiontable.sectorsize);

            // this is home
            disks_export.push(Partition {
                number: 2,
                disk_path: Some(self.selected.clone()),
                path: Some(format!("{}3", self.selected.clone())),
                mountpoint: Some("/home".to_string()),
                filesystem: Some("ext4".to_string()),
                format: true,
                start: last_sector + 1,
                end: current_size_sector.unwrap() - 2048,
                size: current_size_sector.unwrap() - last_sector - 2048,
            });

            // disk larger than 20 GB
        } else if current_size.unwrap() > (20 * 1024 * 1024 * 1024) {
            disks_export.push(Partition {
                number: 0,
                disk_path: Some(self.selected.clone()),
                path: Some(format!("{}1", self.selected.clone())),
                mountpoint: Some("/boot/efi".to_string()),
                filesystem: Some("fat32".to_string()),
                format: true,
                start: 2048, // aligment
                end: 2048 + mb2sector(512, self.partitiontable.partitiontable.sectorsize),
                size: mb2sector(512, self.partitiontable.partitiontable.sectorsize),
            });

            // align + size (prev)
            let last_sector: u64 =
                2048 + mb2sector(512, self.partitiontable.partitiontable.sectorsize);

            // this is root partition
            disks_export.push(Partition {
                number: 1,
                disk_path: Some(self.selected.clone()),
                path: Some(format!("{}2", self.selected.clone())),
                mountpoint: Some("/".to_string()),
                filesystem: Some("ext4".to_string()),
                format: true,
                start: last_sector + 1,
                end: current_size_sector.unwrap() - 2048,
                size: current_size_sector.unwrap() - last_sector - 2048,
            });

            // disk larger than 200 GB
        } else {
            return Err(Box::new(
                error::TealinuxAutoPartitionErr::InsufficientStorage(
                    "Selected storage is lower than 20 GB, Aborted!!".to_string(),
                ),
            ));
        }

        println!("DEBUG current_selected_block_size: {:#?}", current_size);
        println!("{:#?}", disks_export);

        Ok(disks_export)
        // return Err(Box::new(
        //     error::TealinuxAutoPartitionErr::InsufficientStorage(
        //         "something error with getblkbytes()".to_string(),
        //     ),
        // ));

        // if current_size_val < (self.partitiontable.partitiontable.sectorsize * 1024 * 1024) {
        // Err(Box::new(
        //     error::TealinuxAutoPartitionErr::InsufficientStorage(
        //         "check your storages size".to_string(),
        //     ),
        // ))
        // } else {
        //     // ONLY if disk larger than 256 GB
        //     Err(Box::new(
        //         error::TealinuxAutoPartitionErr::InsufficientStorage(
        //             "check your storage size".to_string(),
        //         ),
        //     ))
        // }
    }

    fn _export_data(&self) -> () {
        println!("{:#?}", self.partitiontable);
    }

    // fn convert_block2bytes()
}

#[tauri::command]
pub async fn autogen_partition_select_disk(blkname: String) -> () {
    println!("DEBUG: autoinstall recv {} ", blkname);

    let blkdata = Blkstuff::blockdevice(blkname.clone());
    let disk_result = blkdata.getresult();

    let mut blueprint = super::get_blueprint().unwrap();
    println!("{:#?}", &disk_result);

    blueprint.disk = Some(disk_result.unwrap());
    blueprint._reserved.selected_format_disk = Some(blkname.clone());

    super::write_blueprint(blueprint).unwrap();
}
