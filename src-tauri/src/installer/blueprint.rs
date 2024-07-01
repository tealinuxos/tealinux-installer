use serde::{ Serialize, Deserialize };
use tea_arch_chroot_lib::chroot::Account;
use tea_arch_chroot_lib::chroot::Timezone;
use tea_arch_chroot_lib::chroot::Locale;

#[derive(Debug, Serialize, Deserialize)]
pub struct BluePrint
{
    pub account: Option<Account>,
    pub locale: Option<Locale>,
    pub timezone: Option<Timezone>,
    pub disk: Option<Vec<Partition>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Partition
{
    pub path: Option<String>,
    pub mountpoint: Option<String>,
    pub format: Option<String>,
    pub start: u64,
    pub end: u64,
    pub size: u64
}
