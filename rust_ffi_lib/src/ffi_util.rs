pub(crate) mod samples {
    pub(crate) fn get_safe_str() -> String {
        String::from("Hello World from Rust!")
    }

    pub(crate) fn get_names_vec() -> Vec<String> {
        vec![
            "Bob".to_string(),
            "Sally".to_string(),
            "Mohammed".to_string(),
        ]
    }
}
pub(crate) mod ffi_convert {
    use std::ffi::{c_char, CString};

    #[repr(C)]
    pub struct StringArray {
        ptr: *const *const c_char,
        /// manually set "len" to const
        len: usize,
    }

    pub(crate) unsafe fn to_raw_str(s: String) -> *mut c_char {
        CString::new(s)
            .expect("could not unwrap String to CString")
            .into_raw()
    }

    pub(crate) unsafe fn to_const_raw_str(s: String) -> *const c_char {
        to_raw_str(s) as *const c_char
    }

    pub(crate) unsafe fn free_raw_str(s: *mut c_char) {
        if s.is_null() {
            return;
        }
        unsafe {
            // this line can cause segmentation fault if size of s has changed
            // at any time after "into_raw" has been called!
            let _ = CString::from_raw(s);
        }
    }

    pub(crate) unsafe fn free_const_raw_str(s: *const c_char) {
        free_raw_str(s as *mut c_char);
    }

    pub(crate) unsafe fn to_raw_stringarray(safe_vec: Vec<String>) -> StringArray {
        // capacity == len in this case, since no mods are allowed to occur
        let raw_capacity = safe_vec.len();
        let mut raw_pointer_vec: Vec<*const c_char> = Vec::with_capacity(raw_capacity);

        // Convert each Rust string to CString, then to a raw pointer, and push raw pointer to raw_pointer_vec
        for s in safe_vec {
            raw_pointer_vec.push(to_const_raw_str(s));
        }
        // Get the raw pointer and length of the vector
        let ptr = raw_pointer_vec.as_ptr();
        let len = raw_pointer_vec.len();
        std::mem::forget(raw_pointer_vec); // Prevent Rust from freeing the Vec

        StringArray { ptr, len }
    }

    pub(crate) unsafe fn free_raw_stringarray(array: StringArray) {
        if array.ptr.is_null() {
            return;
        }

        // Convert the raw pointer back into a slice of raw pointers
        let raw_pointer_vec: Vec<*const c_char> =
            Vec::from_raw_parts(array.ptr as *mut *const c_char, array.len, array.len);
        for &raw_ptr in &raw_pointer_vec {
            free_const_raw_str(raw_ptr);
        }
        // raw_pointer_vec is "dropped" i.e. deallocated here after unsafe block ends its scope}
    }
}
