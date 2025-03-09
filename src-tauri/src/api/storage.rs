use super::partition::blueprint_set_partition;
use crate::installer::{ Storage, Partition };

#[tauri::command]
pub async fn blueprint_set_storage(storage: String, partition: String)
{
    let mut storage: Storage = serde_json::from_str(&storage).unwrap();

    let partitions: Vec<Partition> = serde_json::from_str(&partition).unwrap();
    storage.partitions = Some(partitions);

    let mut blueprint = super::get_blueprint().unwrap();

    blueprint.storage = Some(storage);

    super::write_blueprint(blueprint).unwrap();
}
