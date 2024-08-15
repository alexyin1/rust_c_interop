extern crate rust_ffi_lib;

fn main() {
    println!("Hello, world!");
    unsafe {
        let c_like_str = rust_ffi_lib::get_raw_hello_str();
        rust_ffi_lib::free_raw_str(c_like_str);

        let c_like_str_arr = rust_ffi_lib::get_raw_names_vec();
        rust_ffi_lib::free_raw_stringarray(c_like_str_arr);
    }
}
