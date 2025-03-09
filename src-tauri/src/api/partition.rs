use crate::installer::Partition;

#[tauri::command]
pub async fn blueprint_set_partition(partition: String)
{
    let partition: Vec<Partition> = serde_json::from_str(&partition).unwrap();

    let mut blueprint = super::get_blueprint().unwrap();

    // blueprint.disk = Some(partition);

    super::write_blueprint(blueprint).unwrap();
}
