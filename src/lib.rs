extern crate libc;
use libc::{c_char, uint32_t};
use std::ffi::CStr;

#[no_mangle]
pub extern fn addition(a: uint32_t, b: uint32_t) -> uint32_t {
    a + b
}


#[no_mangle]
pub extern fn count_characters(s: *const c_char) -> uint32_t {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };

    let r_str = c_str.to_str().unwrap();
    r_str.chars().count() as uint32_t
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
