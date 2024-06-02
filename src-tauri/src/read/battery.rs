use serde::{ Serialize, Deserialize };
use crate::utils::command::command_with_output;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Battery
{
    capacity: u16
}

impl Battery
{
    pub fn new() -> Self
    {
        let capacity = command_with_output("cat /sys/class/power_supply/BAT1/capacity".to_string());
        let capacity: u16 = capacity.parse().unwrap();

        Self { capacity }
    }
}
