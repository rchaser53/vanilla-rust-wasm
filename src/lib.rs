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

#[test]
fn test() {
    assert!(add(10, -100) == -90);
    assert!(fib(40) == 63245986);
    assert!(get_char() == 'a');
}
