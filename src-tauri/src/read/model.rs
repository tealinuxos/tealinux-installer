use serde::{ Serialize, Deserialize };
use crate::utils::command::command_with_output;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Model
{
    system_product_name: String,
    system_version: String
}

impl Model
{
    pub fn new() -> Self
    {
        let system_product_name = command_with_output("dmidecode -s system-product-name".to_string());
        let system_version = command_with_output("dmidecode -s system-version".to_string());

        Self { system_product_name, system_version }
    }
}
