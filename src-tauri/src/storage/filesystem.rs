pub fn filesystem_list() -> Vec<String>
{
    let filesystem = vec![
        "btrfs",
        "fat32",
        "exfat",
        "ext4",
        "linux-swap(v1)"
    ];

    let filesystem: Vec<String> = filesystem.iter().map(|s| s.to_string()).collect();

    filesystem
}
