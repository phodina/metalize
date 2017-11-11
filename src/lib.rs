extern crate libc;
use libc::{c_char, uint32_t, uint8_t, size_t};
use std::ffi::{CString, CStr};
use std::iter;
use std::slice;


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

#[no_mangle]
pub extern fn create_new_quote(length: uint8_t) -> *mut c_char {
    let mut quote = String::from("💣 ");
    quote.extend(iter::repeat("Because I'm Batman!").take(length as usize));
    quote.push_str(" ka bum 💣");

    let c_str_quote = CString::new(quote).unwrap();
    c_str_quote.into_raw()
}

#[no_mangle]
pub extern fn free_quote(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}

#[no_mangle]
pub extern fn sum(n: *const uint32_t, len: size_t) -> uint32_t {
    let numbers = unsafe {
        assert!(!n.is_null());

        slice::from_raw_parts(n, len as usize)
    };

    let sum =
        numbers.iter()
        .fold(0, |acc, v| acc + v);
    sum as uint32_t
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
