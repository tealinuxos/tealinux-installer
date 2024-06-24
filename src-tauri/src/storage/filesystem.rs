pub fn filesystem_list() -> Vec<String>
{
    let filesystem = vec![
        "btrfs",
        "fat12",
        "fat16",
        "fat32",
        "exfat",
        "ext3",
        "ext4"
    ];

    let filesystem: Vec<String> = filesystem.iter().map(|s| s.to_string()).collect();

    filesystem
}
