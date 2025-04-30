// this impl block below used as converter between tea_partition_generator and vice versa
// because its shares same identical struct, but different signature. so I create the converter

// FROM tea_partition_generatorto tealinux-installer
impl From<tea_partition_generator::blueprint::Partition> for crate::installer::Partition {
    fn from(data: tea_partition_generator::blueprint::Partition) -> Self {
        crate::installer::Partition {
            number: data.number,
            disk_path: data.disk_path,
            path: data.path,
            mountpoint: data.mountpoint,
            filesystem: data.filesystem,
            format: data.format,
            start: data.start,
            end: data.end,
            size: data.size,
        }
    }
}

fn convert_partition(
    opt: Option<Vec<tea_partition_generator::blueprint::Partition>>,
) -> Option<Vec<crate::installer::Partition>> {
    opt.map(|vec| vec.into_iter().map(Into::into).collect())
}

impl From<tea_partition_generator::blueprint::Storage> for crate::installer::Storage {
    fn from(data: tea_partition_generator::blueprint::Storage) -> Self {
        crate::installer::Storage {
            disk_path: data.disk_path,
            partition_table: data.partition_table,
            new_partition_table: data.new_partition_table,
            layout_changed: data.layout_changed,
            partitions: convert_partition(data.partitions),
        }
    }
}
