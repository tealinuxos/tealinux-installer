use tea_partition_api_lib::{ Disk, Partition };
use tea_partition_api_lib::read::get_partition;

pub async fn get_storage() -> Vec<Disk>
{
    get_partition::parted_list_partition()
}
