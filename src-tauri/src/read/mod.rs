pub mod pci;
pub mod model;
pub mod memory;
pub mod storage;

use serde::{ Serialize, Deserialize };
use model::Model;
use memory::Memory;
use pci::Pci;
use storage::disk::Disk;
use storage::partition::Partition;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Read
{
    model: Model,
    memory: Memory,
    storage: Disk,
    operating_systems: Vec<String>,
    battery_percentage: u32,
    online: bool,
    lspci: Pci
}

impl Read
{
    pub fn new(model: Model, memory: Memory, storage: Disk, operating_systems: Vec<String>, battery_percentage:u32, online: bool, lspci: Pci) -> Self
    {
        Self { model, memory, storage, operating_systems, battery_percentage, online, lspci }
    }
}

pub fn get_read() -> Read
{
    // Model
    let model = Model::new("Thinkpad".to_string(), "T480".to_string());

    // Memory
    let memory = Memory::new(8192, 2048);

    // Partitions
    let vec_partitions = vec![
        Partition::new("/dev/sda1".to_string(), 2048, 1050624, 1048576, 512, "ext4".to_string()),
        Partition::new("null".to_string(), 1050625, 5244929, 4194304, 2048, "free".to_string()),
        Partition::new("/dev/sda2".to_string(), 5244930, 22022146, 16777216, 8192, "ext4".to_string())
    ];

    // Storage
    let storage = Disk::new("/dev/sda".to_string(), "gpt".to_string(), vec_partitions);

    let operating_systems = vec!["Windows 10".to_string(), "Arch Linux".to_string()];

    // Pci
    let lspci = Pci::new("Intel(R) Core(TM) i5-8350U CPU @ 1.70GHz".to_string(), "Intel Corporation UHD Graphics 620".to_string());

    Read::new(model, memory, storage, operating_systems, 69, true, lspci)
}
