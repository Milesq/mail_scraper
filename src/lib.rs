mod internal;

use std::{
    ffi::CStr,
    os::raw::{c_char, c_int},
    slice,
};

const CONFIG_SERVER_PORT: &str = "34567";

type CPtrStr = *const c_char;
mod config_server;

#[no_mangle]
pub unsafe extern "C" fn add_record(
    code: CPtrStr,
    file_output: CPtrStr,
    fields_array: *const CPtrStr,
    len: c_int,
) {
    let fields: Vec<_> = slice::from_raw_parts(fields_array, len as usize)
        .iter()
        .map(|el| c_to_string(*el))
        .collect();

    internal::add_record(
        &c_to_string(code),
        &c_to_string(file_output),
        fields.as_slice(),
    )
    .unwrap();
}

unsafe fn c_to_string(c_buf: *const c_char) -> String {
    let c_str: &CStr = CStr::from_ptr(c_buf);
    let str_slice: &str = c_str.to_str().unwrap();
    str_slice.to_owned()
}
