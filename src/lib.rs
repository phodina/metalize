extern crate libc;
use libc::uint32_t;

#[no_mangle]
pub extern fn addition(a: uint32_t, b: uint32_t) -> uint32_t {
    a + b
}

#[allow(dead_code)]
pub extern fn fix_linking_when_not_using_stdlib() { panic!() }

#[cfg(test)]
mod tests {

	use super::*;
	
    #[test]
    fn add_works() {
        assert_eq!(addition(2, 2), 4);
    }
}
