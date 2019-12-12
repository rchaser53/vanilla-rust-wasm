#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

#[no_mangle]
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[no_mangle]
pub fn fib(count: u32) -> u32 {
    if count < 2 {
        return count;
    }

    let (_, ret) = (0..count - 2).fold((0, 1), |(former, ret), _| (ret, ret + former));
    ret
}

#[no_mangle]
pub fn get_char() -> char {
    'a'
}

lazy_static! {
    pub static ref RUST_MEMORY: Mutex<[u8; 1_000]> = Mutex::new([0; 1_000]);
}

#[no_mangle]
pub fn get_u8_array() -> *const u8 {
    let data: Vec<u8> = vec![1,2,3];
    let length = data.len();
    let s = data.as_slice() as &[u8];
    let mut memory = RUST_MEMORY.lock().unwrap();
    memory[..length].clone_from_slice(&s[..length]);

    memory.as_ptr()
}

#[test]
fn test() {
    assert!(add(1, 2) == 3);
    assert!(fib(10) == 34);
    assert!(get_char() == 'a');
}
