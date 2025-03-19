use core::fmt;

#[derive(Debug)]
pub enum TealinuxAutoPartitionErr {
    InsufficientStorage(String),
    InternalErr(String),
}

impl fmt::Display for TealinuxAutoPartitionErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TealinuxAutoPartitionErr::InsufficientStorage(x) => {
                write!(f, "error, insufficient storage, {} bytes", x)
            }
            TealinuxAutoPartitionErr::InternalErr(x) => {
                write!(f, "error, internal error, {}", x)
            }
        }
    }
}

impl std::error::Error for TealinuxAutoPartitionErr {}
