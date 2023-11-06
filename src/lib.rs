use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub unsafe extern "C" fn serialize_project_in_directory(dir: *mut *mut c_char) -> Option<CString> {
    Some(CString::new("result").unwrap())
}