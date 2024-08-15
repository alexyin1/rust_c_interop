mod ffi_util;

use crate::ffi_util::{ffi_convert, samples};
use std::os::raw::c_char;

// note: the string returned from this function cannot be resized in c!
// rust and c have incompatible memory allocators, so all modifications
// requiring resizing must be done in a seperate copy!
// in-place modifications are okay. i.e. "hello" -> "apple"
#[no_mangle]
pub unsafe extern "C" fn get_raw_hello_str() -> *const c_char {
    ffi_convert::to_raw_str(samples::get_safe_str())
}

// Function to free the string allocated by Rust
#[no_mangle]
pub unsafe extern "C" fn free_raw_str(s: *const c_char) {
    ffi_convert::free_const_raw_str(s);
}

#[no_mangle]
pub unsafe extern "C" fn get_raw_names_vec() -> ffi_convert::StringArray {
    ffi_convert::to_raw_stringarray(samples::get_names_vec())
}

// Function to free the `Vec<String>` that was passed to C
#[no_mangle]
pub unsafe extern "C" fn free_raw_stringarray(array: ffi_convert::StringArray) {
    ffi_convert::free_raw_stringarray(array);
}
