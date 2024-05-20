pub fn storage_capacity() -> u64
{
    512000
}

pub fn get_partitions() -> Vec<String>
{
    vec![String::from("/dev/sda1"), String::from("/dev/sda2"), String::from("/dev/sda3")]
}

pub fn get_flags(partition: String) -> Vec<String>
{
    vec![String::from("esp"), String::from("msftdata")]
}

pub fn get_mountpoints(partition: String) -> Vec<String>
{
    vec![String::from("/")]
}

pub fn get_filesystem(partition: String) -> String
{
    String::from("BTRFS")
}
