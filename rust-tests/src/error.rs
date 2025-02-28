use core::fmt;

#[derive(Debug)]
pub enum TealinuxAutoPartitionErr {
    InsufficientStorage(String),
}

impl fmt::Display for TealinuxAutoPartitionErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TealinuxAutoPartitionErr::InsufficientStorage(x) => {
                write!(f, "error, insufficient storage, {} bytes", x)
            }
        }
    }
}

impl std::error::Error for TealinuxAutoPartitionErr {}
