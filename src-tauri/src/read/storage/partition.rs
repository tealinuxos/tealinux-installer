use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Partition
{
    block_name: String,
    start_sector: u128,
    end_sector: u128,
    sector_size: u128,
    size: u128,
    filesystem_type: String
}

impl Partition
{
    pub fn new(block_name: String, start_sector: u128, end_sector: u128, sector_size: u128, size: u128, filesystem_type: String) -> Self
    {
        Self { block_name, start_sector, end_sector, sector_size, size, filesystem_type }
    }
}
