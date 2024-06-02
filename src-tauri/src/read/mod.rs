pub mod pci;
pub mod model;
pub mod memory;
pub mod storage;
pub mod battery;

use serde::{ Serialize, Deserialize };
use model::Model;
use memory::Memory;
use battery::Battery;
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
    battery: Battery,
    online: bool,
    lspci: Pci
}

impl Read
{
    pub fn new(model: Model, memory: Memory, storage: Disk, operating_systems: Vec<String>, battery: Battery, online: bool, lspci: Pci) -> Self
    {
        Self { model, memory, storage, operating_systems, battery, online, lspci }
    }
}

pub fn get_read() -> Read
{
    // Model
    let model = Model::new();

    // Memory
    let memory = Memory::new();

    // Partitions
    let vec_partitions = vec![
        Partition::new("/dev/sda1".to_string(), 2048, 1050624, 1048576, 512, "ext4".to_string()),
        Partition::new("null".to_string(), 1050625, 5244929, 4194304, 2048, "free".to_string()),
        Partition::new("/dev/sda2".to_string(), 5244930, 22022146, 16777216, 8192, "ext4".to_string())
    ];

    // Storage
    let storage = Disk::new("/dev/sda".to_string(), "gpt".to_string(), vec_partitions);

    let operating_systems = vec!["Windows 10".to_string(), "Arch Linux".to_string()];

    // Battery
    let battery = Battery::new();

    // Pci
    let lspci = Pci::new();

    Read::new(model, memory, storage, operating_systems, battery, true, lspci)
}
