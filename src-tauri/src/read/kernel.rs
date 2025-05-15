use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Kernel
{
    name: String,
    version: String
}

impl Kernel
{
    pub fn new() -> Self
    {
        let name = duct::cmd!("uname", "--kernel-name")
            .read()
            .unwrap_or(String::from("Unknown"));

        let version = duct::cmd!("uname", "--kernel-release")
            .read()
            .unwrap_or(String::from("Unknown"));

        Self { name, version }
    }
}

