use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct OperatingSystem
{
    architecture: Option<String>,
    version: Option<String>,
    name: Option<String>
}

impl OperatingSystem
{
    pub fn new() -> Self
    {
        let info = os_info::get();

        let architecture = info.architecture().map(|s| s.to_string());
        let version = Some(info.version().to_string());
        let name = Some(info.os_type().to_string());

        Self { architecture, version, name }
    }
}
