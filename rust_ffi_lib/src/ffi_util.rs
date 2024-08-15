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
    /// c struct for NamespaceIdent.inner()
    pub struct StringArray {
        ptr: *const *const c_char,
        /// manually set "len" to const
        len: usize,
    }

    /// c struct for Vec<NamespaceIdent>
    /// an array of stringarrays
    #[repr(C)]
    pub struct NamespaceIdentArray {
        ptr: *const StringArray,
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

    pub(crate) unsafe fn to_raw_stringarray(str_vec: Vec<String>) -> StringArray {
        // capacity == len in this case, since no mods are allowed to occur
        let mut raw_str_vec: Vec<*const c_char> = Vec::with_capacity(str_vec.len());

        // Convert each Rust string to CString, then to a raw pointer, and push raw pointer to raw_str_vec
        for s in str_vec {
            raw_str_vec.push(to_const_raw_str(s));
        }
        // Get the raw pointer and length of the vector
        let ptr = raw_str_vec.as_ptr();
        let len = raw_str_vec.len();
        std::mem::forget(raw_str_vec); // Prevent Rust from freeing the Vec

        StringArray { ptr, len }
    }

    pub(crate) unsafe fn free_raw_stringarray(array: StringArray) {
        if array.ptr.is_null() {
            return;
        }

        // Convert the raw pointer back into a slice of raw pointers
        let raw_str_vec: Vec<*const c_char> =
            Vec::from_raw_parts(array.ptr as *mut *const c_char, array.len, array.len);
        for raw_ptr in raw_str_vec {
            free_const_raw_str(raw_ptr);
        }
        // raw_str_vec is "dropped" i.e. deallocated here after unsafe block ends its scope}
    }

    pub(crate) unsafe fn to_const_raw_ns_arr(
        ns_vec: Vec<iceberg::NamespaceIdent>,
    ) -> NamespaceIdentArray {
        let mut raw_ns_vec: Vec<StringArray> = Vec::with_capacity(ns_vec.len());
        for ns in ns_vec {
            raw_ns_vec.push(to_raw_stringarray(ns.inner()));
        }

        let ptr = raw_ns_vec.as_ptr();
        let len = raw_ns_vec.len();
        std::mem::forget(raw_ns_vec); // Prevent Rust from freeing the Vec

        NamespaceIdentArray { ptr, len }
    }

    pub(crate) unsafe fn free_raw_ns_array(array: NamespaceIdentArray) {
        if array.ptr.is_null() {
            return;
        }

        // Convert the raw pointer back into a slice of raw pointers
        let raw_ns_vec: Vec<StringArray> =
            Vec::from_raw_parts(array.ptr as *mut StringArray, array.len, array.len);
        for raw_str_vec in raw_ns_vec {
            free_raw_stringarray(raw_str_vec);
        }
        // raw_str_vec is "dropped" i.e. deallocated here after unsafe block ends its scope}
    }
}

pub(crate) mod blocking_iceberg {
    use iceberg::Catalog;
    use iceberg::NamespaceIdent;
    use iceberg_catalog_rest::{RestCatalog, RestCatalogConfig};
    use tokio::runtime::Runtime;

    pub(crate) fn list_namespaces() -> Vec<NamespaceIdent> {
        let config = RestCatalogConfig::builder()
            .uri("http://localhost:8181".to_string())
            .build();

        let my_catalog = RestCatalog::new(config);
        let rt = Runtime::new().unwrap();
        let all_namespaces = rt.block_on(my_catalog.list_namespaces(None)).unwrap();
        all_namespaces
    }
}
