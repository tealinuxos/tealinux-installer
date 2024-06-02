use serde::{ Serialize, Deserialize };
use sysinfo::{ System, RefreshKind, MemoryRefreshKind };

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Memory
{
    capacity: u64,
    used: u64
}

impl Memory
{
    pub fn new() -> Self
    {
        let sysinfo = System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram())
        );

        let capacity = sysinfo.total_memory() / 1024000;
        let used = sysinfo.used_memory() / 1024000;

        Self { capacity, used }
    }
}

