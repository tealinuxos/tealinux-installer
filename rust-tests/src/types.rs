#[derive(Serialize, Deserialize, Debug)]
pub struct Partitions {
    node: String,
    start: u64,
    size: u64,
    #[serde(rename = "type")]
    _type: String,
    #[serde(default)]
    bootable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlkiIitialData {
    label: String,
    id: String,
    device: String,
    unit: String,
    sectorsize: u64,
    partitions: Vec<Partitions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartitionTable {
    partitiontable: BlkiIitialData,
}

#[derive(Debug)]
pub struct Blkstuff {
    selected: String,
    partitiontable: PartitionTable,
}