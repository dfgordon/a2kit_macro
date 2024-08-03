use a2kit_macro::{DiskStruct,DiskStructError};
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
    test_struct = TestStruct::from_bytes(&vec![0,1,2,4,5]).expect("disk struct err");
    v = test_struct.to_bytes();
    assert_eq!(v,vec![0,1,2,4,5]);
}

#[test]
fn out_of_data() {
    match TestStruct::from_bytes(&vec![0,1,2,4]) {
        Err(DiskStructError::OutOfData) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn adv_ptr() {
    let mut ptr = 10;
    let test_struct = TestStruct::from_bytes_adv(&vec![0,1,2,4,5],&mut ptr).expect("disk struct err");
    let v = test_struct.to_bytes();
    assert_eq!(v,vec![0,1,2,4,5]);
    assert_eq!(ptr,15);
}