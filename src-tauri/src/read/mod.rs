pub mod pci;
pub mod model;
pub mod memory;
pub mod battery;
pub mod online;
pub mod desktop_environment;
pub mod operating_system;
pub mod kernel;
pub mod display_server;

use serde::Serialize;
use model::Model;
use memory::Memory;
use battery::Battery;
use online::Online;
use pci::Pci;
use desktop_environment::DesktopEnvironment;
use operating_system::OperatingSystem;
use kernel::Kernel;
use display_server::DisplayServer;
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
    firmware: FirmwareKind,
    desktop_environment: DesktopEnvironment,
    operating_system: OperatingSystem,
    kernel: Kernel,
    display_server: DisplayServer
}

impl Read
{
    pub fn new(model: Model, memory: Memory, disk: Vec<Disk>, battery: Battery, online: Online, lspci: Pci, firmware: FirmwareKind, desktop_environment: DesktopEnvironment, operating_system: OperatingSystem, kernel: Kernel, display_server: DisplayServer) -> Self
    {
        Self { model, memory, disk, battery, online, lspci, firmware, desktop_environment, operating_system, kernel, display_server }
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

    // Desktop Environment
    let desktop_environment = DesktopEnvironment::new();

    // Operating System
    let operating_system = OperatingSystem::new();

    // Kernel
    let kernel = Kernel::new();

    // Display Server
    let display_server = DisplayServer::new();

    Read::new(model, memory, disk, battery, online, lspci, firmware, desktop_environment, operating_system, kernel, display_server)
}
