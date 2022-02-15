use crate::{error::Error, get_xor_mapped_address as _internal_get_xor_mapped_address, Options};
use std::{
    ffi::{CStr, CString},
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
) -> Response {
    #[cfg(debug_assertions)]
    println!("Entering the RUST world");
    let rust_stun_adress = CStr::from_ptr(stun_address).to_str();
    if let Err(error) = rust_stun_adress {
        return Response::error(
            -1,
            format!(
                "Could not instantiate STUN server address String: {}",
                error
            ),
        );
    }
    let stun_address = rust_stun_adress.unwrap().to_socket_addrs();
    if let Err(error) = stun_address {
        return Response::error(
            -2,
            format!("Could not parse STUN server address: {}", error),
        );
    }
    let stun_address = stun_address.unwrap().last();
    if stun_address.is_none() {
        return Response::error(-2, "Could not parse STUN server address".to_string());
    }

    let rust_local_port = CStr::from_ptr(local_port).to_str();
    if let Err(error) = rust_local_port {
        return Response::error(
            -3,
            format!("Could not instantiate local port string: {}", error),
        );
    }
    let options = Options::from(options);
    #[cfg(debug_assertions)]
    println!("Build options correctly, invoking internal function");
    let xor_mapped_result =
        _internal_get_xor_mapped_address(stun_address.unwrap(), rust_local_port.unwrap(), options);
    if let Err(error) = xor_mapped_result {
        let return_value = match error {
            Error::Default(_) => -5,
            Error::Binding(_) => -6,
            Error::Connect(_) => -7,
            Error::Send(_) => -8,
            Error::Receive(_) => -9,
            Error::Parse => -10,
        };
        return Response::error(
            return_value,
            format!("Error while getting XOR mapped address: {}", error),
        );
    }
    let rust_result = xor_mapped_result.unwrap().to_string();
    Response::success(rust_result)
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

#[repr(C)]
pub struct Response {
    status: c_int,
    value: *const c_char,
    error: *const c_char,
}

impl Response {
    unsafe fn success(value: String) -> Response {
        let c_string = CString::new(value).unwrap();
        Response {
            status: 0,
            // https://users.rust-lang.org/t/ffi-and-nested-structs/3586
            value: c_string.into_raw(),
            error: ptr::null_mut(),
        }
    }

    unsafe fn error(status: c_int, error: String) -> Response {
        let c_string = CString::new(error).unwrap();
        Response {
            status,
            value: ptr::null_mut(),
            // https://users.rust-lang.org/t/ffi-and-nested-structs/3586
            error: c_string.into_raw(),
        }
    }
}
