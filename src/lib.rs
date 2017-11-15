extern crate libc;
use libc::{c_char, uint32_t, uint8_t, size_t};
use std::ffi::{CString, CStr};
use std::iter;
use std::slice;
use std::collections::HashMap;
use std::convert::From;

pub use addition::*;
pub use count_characters::*;
pub use quotes::*;
pub use sum::*;
pub use point::*;

pub mod addition {

    use super::*;
    #[no_mangle]
    pub extern fn addition(a: uint32_t, b: uint32_t) -> uint32_t {
        a + b
    }
}

pub mod count_characters {

    use super::*;
    #[no_mangle]
    pub extern fn count_characters(s: *const c_char) -> uint32_t {
        let c_str = unsafe {
            assert!(!s.is_null());

            CStr::from_ptr(s)
        };

        let r_str = c_str.to_str().unwrap();
        r_str.chars().count() as uint32_t
    }
}

pub mod quotes {
    
    use super::*;
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
}

pub mod sum {

    use super::*; 
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
}

pub mod point {

    use super::*;
    // A struct that can be passed between C and Rust
    #[repr(C)]
    pub struct Tuple {
        x: uint32_t,
        y: uint32_t,
    }
    // Conversion functions
    impl From<(u32, u32)> for Tuple {
        fn from(tup: (u32, u32)) -> Tuple {
            Tuple { x: tup.0, y: tup.1 }
        }
    }

    impl From<Tuple> for (u32, u32) {
        fn from(tup: Tuple) -> (u32, u32) {
            (tup.x, tup.y)
        }
    }

    #[no_mangle]
    pub extern fn swap_point_values(tup: Tuple) -> Tuple {
        point_swap(tup.into()).into()
    }

    fn point_swap(point: (u32, u32)) -> (u32, u32) {
        let (a, b) = point;
        (b+1, a-1)
    }

    pub struct AccountDatabase {
        money: HashMap<String, u32>,
    }

    impl AccountDatabase {
        fn new() -> AccountDatabase {
            AccountDatabase {
                money: HashMap::new(),
            }
        }

        fn populate(&mut self) {
            for i in 0..100000 {
                let id = format!("{:05}", i);
                self.money.insert(id, 100000 - i);
            }
        }

        fn current_balance(&self, id: &str) -> u32 {
            self.money.get(id).cloned().unwrap_or(0)
        }
    }

    #[no_mangle]
    pub extern fn account_database_new() -> *mut AccountDatabase {
        Box::into_raw(Box::new(AccountDatabase::new()))
    }

    #[no_mangle]
    pub extern fn account_database_free(ptr: *mut AccountDatabase) {
        if ptr.is_null() { return }
        unsafe { Box::from_raw(ptr); }
    }

    #[no_mangle]
    pub extern fn account_database_populate(ptr: *mut AccountDatabase) {
        let database = unsafe {
            assert!(!ptr.is_null());
            &mut *ptr
        };
        database.populate();
    }

    #[no_mangle]
    pub extern fn account_database_current_balance(ptr: *const AccountDatabase, id: *const c_char) -> uint32_t {
        let database = unsafe {
            assert!(!ptr.is_null());
            &*ptr
        };
        let id = unsafe {
            assert!(!id.is_null());
            CStr::from_ptr(id)
        };
        let id_str = id.to_str().unwrap();
        database.current_balance(id_str)
    }
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
