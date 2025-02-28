pub fn gb2sector(x: u64, sector_size: u64) -> u64 {
    if sector_size == 0 {
        // this is probably non root user
        return 0;
    }
    (x * 1024 * 1024 * 1024) / sector_size
}
