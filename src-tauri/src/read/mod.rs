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
use tea_arch_chroot_lib::resource::FirmwareKind;
use tea_arch_chroot_lib::chroot::bootloader::get_firmware_type;

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct Read
{
    model: Model,
    memory: Memory,
    pub disk: Vec<Disk>,
    battery: Battery,
    online: Online,
    lspci: Pci,
    firmware: FirmwareKind
}

impl Read
{
    pub fn new(model: Model, memory: Memory, disk: Vec<Disk>, battery: Battery, online: Online, lspci: Pci, firmware: FirmwareKind) -> Self
    {
        Self { model, memory, disk, battery, online, lspci, firmware }
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
    
    // Firmware
    let firmware = get_firmware_type();

    Read::new(model, memory, disk, battery, online, lspci, firmware)
}
