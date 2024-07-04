use serde::{ Serialize, Deserialize };
use tea_arch_chroot_lib::chroot::Account;
use tea_arch_chroot_lib::chroot::Timezone;
use tea_arch_chroot_lib::chroot::Locale;
use tea_arch_chroot_lib::resource::FirmwareKind;

#[derive(Debug, Serialize, Deserialize)]
pub struct BluePrint
{
    pub account: Option<Account>,
    pub locale: Option<Locale>,
    pub timezone: Option<Timezone>,
    pub disk: Option<Vec<Partition>>,
    pub bootloader: Option<Bootloader>,
    pub keyboard: Option<Keyboard>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Partition
{
    pub path: Option<String>,
    pub mountpoint: Option<String>,
    pub filesystem: Option<String>,
    pub format: bool,
    pub start: u64,
    pub end: u64,
    pub size: u64
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Bootloader
{
    pub firmware_type: FirmwareKind,
    pub path: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Keyboard
{
    pub layout: Option<String>,
    pub variant: Option<String>
}
