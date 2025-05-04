use crate::installer::Partition;
use duct::cmd;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use tea_partition_generator::core::{PartitionGenerator, TeaPartitionGenerator};

// this is lsblk stuff
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

#[derive(Deserialize, Serialize, Debug)]
struct Disklists {
    pub blkname: String,
    pub blksize: String,
}

// end lsblk stuff

#[tauri::command]
pub async fn blueprint_set_partition(partition: String) {
    let partition: Vec<Partition> = serde_json::from_str(&partition).unwrap();

    let mut blueprint = super::get_blueprint().unwrap();

    // blueprint.disk = Some(partition);

    super::write_blueprint(blueprint).unwrap();
}

#[tauri::command]
pub async fn set_auto_config_partition() -> () {
    let mut blueprint: Result<crate::installer::BluePrint, std::io::Error> = super::get_blueprint();

    if let Ok(blueprint_val) = blueprint {
        // blueprint_val.disk = self::get_disk_size().await;
        super::write_blueprint(blueprint_val);
    }
}

async fn _get_disk_lists() -> Vec<Disklists> {
    let disk = cmd!("lsblk", "-J").read();

    let ignlist = vec!["zram0".to_string(), "fd0".to_string()];

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

    println!("{:?}", ret);

    ret
}

#[tauri::command]
pub async fn get_disk_lists_key_val() -> String {
    println!("invoked");
    let ret = self::_get_disk_lists().await;
    let ret = serde_json::to_string(&ret);

    if let Ok(ret_val) = ret {
        ret_val
    } else {
        "[]".to_string()
    }
}

// for js obj
#[derive(Serialize, Deserialize, Debug)]
struct DiskListsKeyValHasOs {
    block_name: String,
    fe_interface: String,
}

#[tauri::command]
pub async fn get_disk_lists_key_val_with_otheros_check() -> String {
    println!("invoked");

    // simple hashmap for json formatting
    // let mut disk_os: HashMap<String, String> = HashMap::new();
    let mut disk_os: Vec<DiskListsKeyValHasOs> = Vec::new();

    let ret = self::_get_disk_lists().await;
    let other_os_lists = TeaPartitionGenerator::disk_list_other_os().await;

    let mut mutex = false;
    for ret_i in ret {
        mutex = false;
        if let Some(ref other_os_lists_val) = other_os_lists {
            for other_os_lists_i in other_os_lists_val {
                if ret_i.blkname.clone() == other_os_lists_i.path {
                    // found
                    let fe_interface = format!("{} ({})", other_os_lists_i.name, ret_i.blksize);
                    // disk_os.insert(ret_i.blkname.clone(), fe_interface);
                    disk_os.push(DiskListsKeyValHasOs {
                        block_name: ret_i.blkname.clone(),
                        fe_interface: fe_interface,
                    });

                    mutex = true;
                }
            }
        }

        if mutex == false {
            let fe_interface = format!("({})", ret_i.blksize);
            // disk_os.insert(ret_i.blkname, fe_interface);

            disk_os.push(DiskListsKeyValHasOs {
                block_name: ret_i.blkname.clone(),
                fe_interface: fe_interface,
            });
        }
    }

    let ret = serde_json::to_string(&disk_os);
    println!("{:#?}", ret);

    if let Ok(ret_val) = ret {
        ret_val
    } else {
        "[]".to_string()
    }
}
