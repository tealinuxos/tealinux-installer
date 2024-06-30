pub mod pci;
pub mod model;
pub mod memory;
pub mod battery;
pub mod online;

use serde::Serialize;
use model::Model;
use memory::Memory;
use battery::Battery;
use online::Online;
use pci::Pci;
use tea_partition_api_lib::{ Disk, Partition };
use tea_partition_api_lib::read::get_partition;

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct Read
{
    model: Model,
    memory: Memory,
    disk: Vec<Disk>,
    battery: Battery,
    online: Online,
    lspci: Pci
}

impl Read
{
    pub fn new(model: Model, memory: Memory, disk: Vec<Disk>, battery: Battery, online: Online, lspci: Pci) -> Self
    {
        Self { model, memory, disk, battery, online, lspci }
    }
}

pub fn get_read() -> Read
{
    // Model
    let model = Model::new();

    // Memory
    let memory = Memory::new();

    // Storage
    let disk = get_partition::parted_list_partition();

    // Battery
    let battery = Battery::new();

    // Online
    let online = Online::new();

    // Pci
    let lspci = Pci::new();

    Read::new(model, memory, disk, battery, online, lspci)
}
