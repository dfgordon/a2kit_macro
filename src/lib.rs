//! # Procedural macro module for a2kit

use core::fmt::Display;
use core::fmt::Debug;

#[derive(Debug)]
pub enum DiskStructError {
    OutOfData,
    UnexpectedSize,
    UnexpectedValue,
    IllegalValue
}

impl Display for DiskStructError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfData => write!(f,"out of data while loading DiskStruct"),
            Self::UnexpectedSize => write!(f,"size of data is not normal"),
            Self::UnexpectedValue => write!(f,"unexpected value while loading DiskStruct"),
            Self::IllegalValue => write!(f,"illegal value while loading DiskStruct")
        }
    }
}

impl std::error::Error for DiskStructError {
}

/// Automatically derived trait for any fixed length disk structure.
/// This trait is used to create them, and move them in and out of buffers.
pub trait DiskStruct {
    /// create a structure with all zeros, requires concrete type
    fn new() -> Self where Self: Sized;
    /// flatten the structure into a byte stream
    fn to_bytes(&self) -> Vec<u8>;
    /// create a structure from a byte stream, requires concrete type
    fn from_bytes(bytes: &[u8]) -> Result<Self, DiskStructError> where Self: Sized;
    /// update a structure from a byte stream, OK to use on trait objects
    fn update_from_bytes(&mut self,bytes: &[u8]) -> Result<(), DiskStructError>;
    /// the length in bytes of the structure
    fn len(&self) -> usize;
    /// convenience function to call `from_bytes` and increment `ptr` by `len()`
    fn from_bytes_adv(bytes: &[u8], ptr: &mut usize) -> Result<Self,DiskStructError> where Self: Sized {
        match Self::from_bytes(bytes) {
            Ok(res) => {
                *ptr += res.len();
                Ok(res)
            }
            Err(e) => Err(e)
        }
    }
}
