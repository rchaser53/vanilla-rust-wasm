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

#[no_mangle]
pub extern fn get_i32_array() -> *const i32 {
    let data = vec![1,2,3];
    let length = data.len();

    // preserve memory
    let mut memory: [i32; 1_00] = [0; 1_00];
    let s = data.as_slice() as &[i32];
    memory[..length].clone_from_slice(&s[..length]);

    memory.as_ptr()
}

#[test]
fn test() {
    assert!(add(1, 2) == 3);
    assert!(fib(10) == 34);
    assert!(get_char() == 'a');
}
