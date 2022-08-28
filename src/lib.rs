pub trait DiskStruct {
    fn new() -> Self;
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &Vec<u8>) -> Self;
    // the following is mainly used to ease the metaprogramming of `from_bytes`
    fn update_from_bytes(&mut self,bytes: &Vec<u8>);
    fn len(&self) -> usize;
}
