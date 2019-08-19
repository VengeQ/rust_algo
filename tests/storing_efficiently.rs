#[repr(C)]
struct NumberStruct1 {
    a: u16,
    b: u32,
    c: u16,
}

struct NumberStruct2 {
    a: u32,
    //4
    b: u32,
    //4
    c: u16, //2 ???
}

use std::mem;

const U32_ADD_DOUBLE_U16_SIZE: usize = mem::size_of::<u32>() + 2 * mem::size_of::<u16>();
//8
const U16_ADD_DOUBLE_U32_SIZE: usize = 2 * mem::size_of::<u32>() + mem::size_of::<u16>(); //10

#[test]
fn stack_allocation_test_1() {
    println!("size of NumberStruct1: {}", mem::size_of::<NumberStruct1>());
    assert_eq!(mem::size_of::<NumberStruct1>(), U32_ADD_DOUBLE_U16_SIZE);
}

#[test]
fn stack_allocation_test_2() {
    println!("\nsize of NumberStruct2: {}", mem::size_of::<NumberStruct2>());
  //  assert_eq!(mem::size_of::<NumberStruct2>(), U16_ADD_DOUBLE_U32_SIZE + 2);
}