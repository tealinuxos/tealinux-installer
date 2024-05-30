use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Memory
{
    capacity: u64,
    used: u64
}

impl Memory
{
    pub fn new(capacity: u64, used: u64) -> Self
    {
        Self { capacity, used }
    }
}

