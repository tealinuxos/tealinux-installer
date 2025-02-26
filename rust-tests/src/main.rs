mod error;
mod partition;

use duct::cmd;
use partition::Blkutils;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, hash::Hash};

#[derive(Serialize, Deserialize, Debug)]
struct LsblkBlockdevicesChildren {
    pub name: String,
    #[serde(rename = "maj:min")]
    pub maj_min: String,
    pub rm: bool,
    pub size: String,
    pub ro: bool,
    #[serde(rename = "type")] // bentrok
    pub disk_type: String,
    pub mountpoints: Vec<Option<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct LsblkBlockdevices {
    pub name: String,
    #[serde(rename = "maj:min")]
    pub maj_min: String,
    pub rm: bool,
    pub size: String,
    pub ro: bool,
    #[serde(rename = "type")] // bentrok
    pub disk_type: String,
    pub mountpoints: Vec<Option<String>>,
    #[serde(default)]
    pub children: Vec<Option<LsblkBlockdevicesChildren>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Lsblk {
    blockdevices: Vec<LsblkBlockdevices>,
}

#[derive(Deserialize, Debug)]
struct Disklists {
    blkname: String,
    blksize: String,
}

fn get_disk_lists() -> Vec<Disklists> {
    let disk = cmd!("lsblk", "-J").read();

    let ignlist = vec!["zram0".to_string()];

    // intiial
    let mut ret: Vec<Disklists> = Vec::new();

    // let mut ret: HashMap<HashMap<String, String>, HashMap<String, String>> = HashMap::new();

    if let Ok(disk_val) = disk {
        let disk_parsed: Lsblk = serde_json::from_str::<Lsblk>(&disk_val).unwrap();

        // pack and ignore

        for disk in disk_parsed.blockdevices.iter() {
            if disk.disk_type == "disk" {
                if !ignlist.contains(&disk.name) {
                    let dev_block = format!("/dev/{}", disk.name);
                    // ret.insert(dev_block, disk.size.clone());

                    ret.push(Disklists {
                        blkname: dev_block,
                        blksize: disk.size.clone(),
                    });
                }
            }
        }
    }

    ret

    // if let Ok(blueprint_val) = blueprint {
    //     // blueprint_val.disk = self::get_disk_size().await;
    //     super::write_blueprint(blueprint_val);
    // }
}

fn main() {
    // println!("Hello, world!");
    // let disk_parsed = self::get_disk_lists();
    // println!("{:#?}", disk_parsed);

    let blkdata = partition::Blkstuff::blockdevice("/dev/sda".to_string());
    let devblock = blkdata.getblkbytes();
    println!("{:#?}", devblock);
}
