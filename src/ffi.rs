// contains ffi for c++
use super::client_api;
use std::ffi::CStr;
use std::ptr;
use libc::c_char;


#[no_mangle]
pub unsafe extern "C" fn sign_in_from_c (client_id: *const c_char) -> *mut String {

    let raw = CStr::from_ptr(client_id);
    let client_id_string = match raw.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let resp = match client_api::sync_sign_in(&String::from(client_id_string)) {
        Ok(s) => s,
        Err(_) => String::from("Sign in failed"),
    };
    
    Box::into_raw(Box::new(resp))
}

#[no_mangle]
pub unsafe extern "C" fn sign_in_clean_up (resp: *mut String) {
    if !resp.is_null() {
        drop(Box::from_raw(resp));
    }
}


