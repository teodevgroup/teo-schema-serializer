use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::PathBuf;
use std::ptr::null_mut;
use std::str::FromStr;
use teo_runtime::namespace::Namespace;
use teo_runtime::schema::load::load_schema::load_schema;
use teo_runtime::stdlib::load::{load as load_std};
use teo_runtime::utils::find_main_schema_file;
use teo_parser::parse;
use serde_json;

#[no_mangle]
pub unsafe extern "C" fn serialize_project_in_directory(dir: *const c_char, error_code: *mut u8) -> *mut c_char {
    let dir_c: &CStr = CStr::from_ptr(dir);
    let dir_rust = dir_c.to_str().unwrap().to_string();
    let dir_without_prefix = if let Some(result) = dir_rust.strip_prefix("file://") {
        result
    } else {
        dir_rust.as_str()
    };
    let Ok(main_schema_file) = find_main_schema_file(None, &PathBuf::from_str(dir_without_prefix).unwrap()) else {
        *error_code = 1;
        return null_mut();
    };
    let (schema, diagnostics) = parse(main_schema_file.as_path().to_str().unwrap(), None, None);
    if diagnostics.has_errors() {
        *error_code = 2;
        return null_mut();
    }
    let mut namespace = Namespace::main();
    load_std(&mut namespace);
    if load_schema(&mut namespace, &schema, true).is_err() {
        *error_code = 3;
        return null_mut();
    }
    match serde_json::to_string(&namespace) {
        Ok(string) => {
            let c_string = CString::new(string).unwrap();
            *error_code = 0;
            c_string.into_raw()
        },
        Err(err) => {
            println!("{err}");
            *error_code = 4;
            return null_mut();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn free_serialize_project_in_directory_output(result: *mut c_char) {
    if !result.is_null() {
        let _ = CString::from_raw(result);
    }
}