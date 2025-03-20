use serde::{Deserialize, Serialize};
use tea_arch_chroot_lib::chroot::Account;
use tea_arch_chroot_lib::chroot::Locale;
use tea_arch_chroot_lib::chroot::Timezone;
use tea_arch_chroot_lib::resource::FirmwareKind;

#[derive(Debug, Serialize, Deserialize)]
pub struct Reserved {
    pub selected_format_disk: Option<String>, // inform which disk should be formatted
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BluePrint {
    pub account: Option<Account>,
    pub locale: Option<Locale>,
    pub timezone: Option<Timezone>,
    pub storage: Option<Storage>,
    pub bootloader: Option<Bootloader>,
    pub keyboard: Option<Keyboard>,
    pub _reserved: Reserved,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Partition {
    pub number: u64,
    pub disk_path: Option<String>,
    pub path: Option<String>,
    pub mountpoint: Option<String>,
    pub filesystem: Option<String>,
    pub format: bool,
    pub start: u64,
    pub end: u64,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bootloader {
    pub firmware_type: FirmwareKind,
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keyboard {
    pub layout: Option<String>,
    pub variant: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Storage
{
    pub disk_path: Option<String>,
    pub partition_table: Option<String>,
    pub new_partition_table: bool,
    pub layout_changed: bool,
    pub partitions: Option<Vec<Partition>>
}
