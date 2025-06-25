use uuid::Uuid;
use std::ffi::CString;
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn generate_uuid_v4() -> *mut c_char {
    let uuid = Uuid::new_v4().to_string();
    let c_str = CString::new(uuid).unwrap();
    c_str.into_raw() // Caller must free this!
}

#[unsafe(no_mangle)]
pub extern "C" fn free_uuid(ptr: *mut c_char) {
    if ptr.is_null() { return; }
    unsafe { CString::from_raw(ptr); } // drops and frees
}
