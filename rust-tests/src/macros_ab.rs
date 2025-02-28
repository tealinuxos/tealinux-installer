pub fn gb2sector(x: u64, sector_size: u64) -> u64 {
    if sector_size == 0 {
        // this is probably non root user
        return 0;
    }
    (x * 1024 * 1024 * 1024) / sector_size
}

pub fn mb2sector(x: u64, sector_size: u64) -> u64 {
    if sector_size == 0 {
        // this is probably non root user
        return 0;
    }
    (x * 1024 * 1024) / sector_size
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_calculate_size() {
//         assert_eq!(calculate_size!(1), 2097152);
//         assert_eq!(calculate_size!(2), 4194304);
//         assert_eq!(calculate_size!(5), 10485760);
//     }
// }
