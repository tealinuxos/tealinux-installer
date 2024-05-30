use serde::{ Serialize, Deserialize };
use super::partition::Partition;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Disk
{
    name: String,
    partition_table_type: String,
    partitions: Vec<Partition>
}

impl Disk
{
    pub fn new(name: String, partition_table_type: String, partitions: Vec<Partition>) -> Self
    {
        Self { name, partition_table_type, partitions }
    }
}
