pub mod pci;
pub mod model;
pub mod memory;
pub mod storage;
pub mod battery;
pub mod online;

use serde::Serialize;
use model::Model;
use memory::Memory;
use battery::Battery;
use online::Online;
use pci::Pci;
use parted_information_extractor_lib::gagah_disk::{ Disk, get_disk_from_json::get_all_disk };

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct Read
{
    model: Model,
    memory: Memory,
    disks: Vec<Disk>,
    operating_systems: Vec<String>,
    battery: Battery,
    online: Online,
    lspci: Pci
}

impl Read
{
    pub fn new(model: Model, memory: Memory, disks: Vec<Disk>, operating_systems: Vec<String>, battery: Battery, online: Online, lspci: Pci) -> Self
    {
        Self { model, memory, disks, operating_systems, battery, online, lspci }
    }
}

pub fn get_read() -> Read
{
    // Model
    let model = Model::new();

    // Memory
    let memory = Memory::new();

    // Operating Systems
    let operating_systems = vec!["Windows 10".to_string(), "Arch Linux".to_string()];

    // Storage
    let json = get_all_disk();
    let storage = Disk::from_value_vec(json);

    // Battery
    let battery = Battery::new();

    // Online
    let online = Online::new();

    // Pci
    let lspci = Pci::new();

    Read::new(model, memory, storage, operating_systems, battery, online, lspci)
}
