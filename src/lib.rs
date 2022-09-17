//! # Procedural macro module for a2kit

/// Automatically derived trait for any fixed length disk structure.
/// This trait is used to create them, and move them in and out of buffers.
pub trait DiskStruct {
    /// create a structure with all zeros, requires concrete type
    fn new() -> Self where Self: Sized;
    /// flatten the structure into a byte stream
    fn to_bytes(&self) -> Vec<u8>;
    /// create a structure from a byte stream, requires concrete type
    fn from_bytes(bytes: &Vec<u8>) -> Self where Self: Sized;
    /// update a structure from a byte stream, OK to use on trait objects
    fn update_from_bytes(&mut self,bytes: &Vec<u8>);
    /// the length in bytes of the structure
    fn len(&self) -> usize;
}
