use super::commands::*;

#[tauri::command]
pub async fn get_region() -> Vec<String>
{
    let args = format!("tree /usr/share/zoneinfo -L 1 -dai --noreport");
    let output = command_with_output(args);

    let vec_output: Vec<&str> = output
        .split('\n')
        .collect();

    let mut vec_output: Vec<String> = vec_output
        .iter()
        .map(|vec| vec.to_string())
        .collect();

    vec_output.remove(0);

    vec_output
}

#[tauri::command]
pub async fn get_city(region: String) -> Vec<String>
{
    let args = format!("tree /usr/share/zoneinfo/{} -ai --noreport", region);
    let output = command_with_output(args);

    let vec_output: Vec<&str> = output
        .split('\n')
        .collect();

    let mut vec_output: Vec<String> = vec_output
        .iter()
        .map(|vec| vec.to_string())
        .collect();

    vec_output.remove(0);

    vec_output
}

#[tauri::command]
pub async fn generate_localtime(region: String, city: String) -> bool
{
    let cmd = format!("ln -sf /usr/share/zoneinfo/{}/{} /etc/localtime", region, city);
    let status = command_with_status(cmd);

    status
}
