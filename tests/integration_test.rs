use a2kit_macro::DiskStruct;
use a2kit_macro_derive::DiskStruct;

#[derive(DiskStruct)]
struct TestStruct {
    f1: u8,
    f2: [u8;4]
}

#[test]
fn test_all() {
    let mut test_struct = TestStruct { 
        f1: 6, 
        f2: [5,0xc,0,2]
    };
    let mut v = test_struct.to_bytes();
    assert_eq!(v,vec![6,5,12,0,2]);
    assert_eq!(5,test_struct.len());
    test_struct = TestStruct::from_bytes(&vec![0,1,2,4,5]);
    v = test_struct.to_bytes();
    assert_eq!(v,vec![0,1,2,4,5]);
}