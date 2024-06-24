use serde::{ Serialize, Deserialize };
use tea_arch_chroot_lib::chroot::Account;
use tea_arch_chroot_lib::chroot::Timezone;
use tea_arch_chroot_lib::chroot::Locale;

#[derive(Debug, Serialize, Deserialize)]
pub struct BluePrint
{
    pub account: Account,
    pub locale: Locale,
    pub timezone: Timezone,
    pub partition: Vec<Partition>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Partition
{
    pub path: String,
    pub mountpoint: Option<String>,
    pub format: Option<String>
}
