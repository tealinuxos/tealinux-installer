use super::partition::blueprint_set_partition;
use crate::{api::{get_read_from_opt, set_read_json, write_read}, installer::{ Partition, Storage }, read::Read};
use std::io::Error;
use tea_partition_api_lib::read::get_partition::parted_list_partition;

#[tauri::command]
pub async fn blueprint_set_storage(storage: String)
{
    let storage: Storage = serde_json::from_str(&storage).unwrap();

    println!("{:#?}", storage);

    // let partitions: Vec<Partition> = serde_json::from_str(&partition).unwrap();
    // storage.partitions = Some(partitions);

    let mut blueprint = super::get_blueprint().unwrap();

    blueprint.storage = Some(storage);

    super::write_blueprint(blueprint).unwrap();
}

#[tauri::command]
pub async fn read_refresh_disk()
{
    let fresh_disk = parted_list_partition();

    let read = get_read_from_opt().await;
    let mut read: Read = serde_json::from_str(&read).expect("Failed to parse read");
    read.disk = fresh_disk;

    write_read(read);
}
