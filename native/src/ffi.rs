use crate::{get_xor_mapped_address as _internal_get_xor_mapped_address, Options};
use std::{
    alloc::{alloc, Layout},
    ffi::{CStr, CString},
    mem,
    net::ToSocketAddrs,
    os::raw::{c_char, c_int, c_long},
    ptr,
    time::Duration,
};

/// # Safety
///
/// Watch out.
#[no_mangle]
#[allow(unused_assignments, unused_variables)]
pub unsafe extern "C" fn get_xor_mapped_address(
    stun_address: *const c_char,
    local_port: *const c_char,
    options: COptions,
    result: *mut *mut c_char,
) -> c_int {
    #[cfg(debug_assertions)]
    println!("Entering the RUST world");
    let rust_stun_adress = CStr::from_ptr(stun_address).to_str();
    if rust_stun_adress.is_err() {
        return -1;
    }
    let stun_address = rust_stun_adress.unwrap().to_socket_addrs();
    if stun_address.is_err() {
        return -2;
    }
    let stun_address = stun_address.unwrap().last();
    if stun_address.is_none() {
        return -2;
    }

    let rust_local_port = CStr::from_ptr(local_port).to_str();
    if rust_local_port.is_err() {
        return -3;
    }
    let options = Options::from(options);
    #[cfg(debug_assertions)]
    println!("Build options correctly, invoking internal function");
    let xor_mapped_result =
        _internal_get_xor_mapped_address(stun_address.unwrap(), rust_local_port.unwrap(), options);
    if xor_mapped_result.is_err() {
        return -4;
    }
    let rust_result = xor_mapped_result.unwrap().to_string();
    let length = rust_result.len();
    let c_string = CString::new(rust_result).unwrap();
    let char_ptr = c_string.as_ptr();
    let allocation = alloc(
        Layout::from_size_align(length, mem::align_of::<c_char>()).expect("Could align memory"),
    );
    // Generates a segmentation
    *result = allocation as *mut c_char;
    ptr::copy(char_ptr, *result, length);
    0
}

#[repr(C)]
pub struct COptions {
    timeout: *const CDuration,
    software: *const c_char, // may be NULL
}

impl From<COptions> for Options {
    fn from(c_options: COptions) -> Self {
        unsafe {
            let software: Option<String> = if c_options.software.is_null() {
                None
            } else {
                Some(
                    CStr::from_ptr(c_options.software)
                        .to_str()
                        .expect("Could not decode software")
                        .to_string(),
                )
            };
            let c_duration = &*c_options.timeout;
            let duration: Option<Duration> = if c_options.timeout.is_null() {
                None
            } else {
                Some(Duration::from(c_duration))
            };
            Options::new(duration, software)
        }
    }
}

#[repr(C)]
pub struct CDuration {
    secs: c_long,
    nanos: c_int,
}

impl From<&CDuration> for Duration {
    fn from(c_duration: &CDuration) -> Self {
        let secs: u64 = c_duration.secs.try_into().unwrap_or_else(|_| {
            panic!(
                "Could not convert CDuration.seconds to Duration.seconds : {}",
                c_duration.secs
            )
        });
        let nanos: u32 = c_duration.nanos.try_into().unwrap_or_else(|_| {
            panic!(
                "Could not convert CDuration.nanos to Duration.nanos : {}",
                c_duration.nanos
            )
        });
        Duration::new(secs, nanos)
    }
}
