//! Helper functions for FFI operations

use crate::error::{SzError, SzResult};
use libc::{c_char, size_t};
use std::ffi::{CStr, CString};
use std::ptr;

/// Converts a Rust string to a C string
pub fn str_to_c_string(s: &str) -> SzResult<CString> {
    CString::new(s).map_err(SzError::from)
}

/// Converts an optional Rust string to a C string pointer
pub fn optional_str_to_c_ptr(s: Option<&str>) -> SzResult<*const c_char> {
    match s {
        Some(s) => {
            let c_string = str_to_c_string(s)?;
            // We need to leak this string to keep it alive for the FFI call
            Ok(c_string.into_raw() as *const c_char)
        }
        None => Ok(ptr::null()),
    }
}

/// Converts a C string pointer to a Rust string and frees the C string
///
/// # Safety
///
/// The caller must ensure that `ptr` is either null or a valid pointer to a null-terminated C string
/// that was allocated by the Senzing library and can be freed with `Sz_free`.
pub unsafe fn c_str_to_string(ptr: *mut c_char) -> SzResult<String> {
    if ptr.is_null() {
        return Ok(String::new());
    }

    let c_str = unsafe { CStr::from_ptr(ptr) };
    let result = match c_str.to_str() {
        Ok(s) => Ok(s.to_string()),
        Err(_) => {
            // If the C string contains invalid UTF-8, convert it to hex encoding to preserve binary data
            // This handles cases where Senzing returns binary data or handles
            let bytes = c_str.to_bytes();
            Ok(hex::encode(bytes))
        }
    };

    // Free the C string memory using Senzing's free function
    unsafe { super::bindings::Sz_free(ptr) };

    result
}

/// Converts C string to Rust string without freeing the memory (for static/managed strings)
///
/// # Safety
///
/// The caller must ensure that ptr is either null or points to a valid
/// null-terminated C string. This function does NOT free the memory.
pub unsafe fn c_str_to_string_no_free(ptr: *mut c_char) -> SzResult<String> {
    if ptr.is_null() {
        return Ok(String::new());
    }

    let c_str = unsafe { CStr::from_ptr(ptr) };
    match c_str.to_str() {
        Ok(s) => Ok(s.to_string()),
        Err(_) => {
            // If the C string contains invalid UTF-8, convert it to hex encoding to preserve binary data
            let bytes = c_str.to_bytes();
            Ok(hex::encode(bytes))
        }
    }
}

/// Processes an SzPointerResult from helper functions
///
/// # Safety
///
/// The caller must ensure that the SzPointerResult contains valid data
pub unsafe fn process_pointer_result(result: super::bindings::SzPointerResult) -> SzResult<String> {
    if result.return_code != 0 {
        return Err(SzError::from_code(result.return_code));
    }

    unsafe { c_str_to_string(result.response) }
}

/// Processes an SzPointerResult from config helper functions
///
/// # Safety
///
/// The caller must ensure that the SzPointerResult contains valid data
/// and that any response pointer is valid and null-terminated.
pub unsafe fn process_config_pointer_result(
    result: super::bindings::SzPointerResult,
) -> SzResult<String> {
    if result.return_code != 0 {
        check_config_return_code(result.return_code)?;
        unreachable!("check_config_return_code should have returned an error");
    }

    unsafe { c_str_to_string(result.response) }
}

/// Processes an SzPointerResult from config helper functions and returns raw bytes
///
/// # Safety
///
/// The caller must ensure that the SzPointerResult contains valid data
/// and that any response pointer is valid.
pub unsafe fn process_config_pointer_result_bytes(
    result: super::bindings::SzPointerResult,
) -> SzResult<Vec<u8>> {
    if result.return_code != 0 {
        check_config_return_code(result.return_code)?;
        unreachable!("check_config_return_code should have returned an error");
    }

    unsafe { c_str_to_bytes(result.response) }
}

/// Processes an SzPointerResult from engine helper functions
///
/// # Safety
///
/// The caller must ensure that the SzPointerResult contains valid data
/// and that any response pointer is valid and null-terminated.
pub unsafe fn process_engine_pointer_result(
    result: super::bindings::SzPointerResult,
) -> SzResult<String> {
    if result.return_code != 0 {
        check_return_code(result.return_code)?;
        unreachable!("check_return_code should have returned an error");
    }

    unsafe { c_str_to_string(result.response) }
}

/// Converts C string to raw bytes for handle storage
///
/// # Safety
///
/// The caller must ensure that ptr is either null or points to a valid
/// null-terminated C string that can be safely freed with Sz_free.
pub unsafe fn c_str_to_bytes(ptr: *mut c_char) -> SzResult<Vec<u8>> {
    if ptr.is_null() {
        return Ok(Vec::new());
    }

    let c_str = unsafe { CStr::from_ptr(ptr) };
    let bytes = c_str.to_bytes().to_vec();

    // Free the C string memory using Senzing's free function
    unsafe { super::bindings::Sz_free(ptr) };

    Ok(bytes)
}

/// Processes an SzPointerResult from config manager helper functions
///
/// # Safety
///
/// The caller must ensure that the SzPointerResult contains valid data
/// and that any response pointer is valid and null-terminated.
pub unsafe fn process_config_mgr_pointer_result(
    result: super::bindings::SzPointerResult,
) -> SzResult<String> {
    if result.return_code != 0 {
        check_config_mgr_return_code(result.return_code)?;
        unreachable!("check_config_mgr_return_code should have returned an error");
    }

    unsafe { c_str_to_string(result.response) }
}

/// Processes an SzLongResult from config manager helper functions
pub fn process_config_mgr_long_result(result: super::bindings::SzLongResult) -> SzResult<i64> {
    if result.return_code != 0 {
        check_config_mgr_return_code(result.return_code)?;
        unreachable!("check_config_mgr_return_code should have returned an error");
    }

    Ok(result.response)
}

/// Processes an SzLongResult from helper functions
pub fn process_long_result(result: super::bindings::SzLongResult) -> SzResult<i64> {
    if result.return_code != 0 {
        return Err(SzError::from_code(result.return_code));
    }

    Ok(result.response)
}

/// Handles memory allocation for FFI response strings
pub struct ResponseBuffer {
    pub ptr: *mut c_char,
    pub size: size_t,
}

impl Default for ResponseBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl ResponseBuffer {
    pub fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            size: 0,
        }
    }

    pub fn as_string(&self) -> SzResult<String> {
        unsafe { c_str_to_string(self.ptr) }
    }
}

impl Drop for ResponseBuffer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                super::bindings::Sz_free(self.ptr);
            }
        }
    }
}

/// Checks the return code from Senzing FFI functions
pub fn check_return_code(return_code: i64) -> SzResult<()> {
    if return_code == 0 {
        Ok(())
    } else {
        let mut buffer: Vec<u8> = vec![0; 1024];
        let buffer_len = buffer.len() as i64;

        let error_msg = unsafe {
            let actual_len =
                super::bindings::Sz_getLastException(buffer.as_mut_ptr() as *mut i8, buffer_len);

            if actual_len > 0 && actual_len < buffer_len {
                // Sz_getLastException returns length including null terminator
                // Exclude the null terminator from the string content
                let string_len = if actual_len > 0 {
                    (actual_len as usize) - 1
                } else {
                    0
                };
                buffer.truncate(string_len);
                String::from_utf8_lossy(&buffer).to_string()
            } else {
                format!("Unknown error (code: {})", return_code)
            }
        };

        // Create error with proper Senzing message - backtrace is now captured automatically
        match return_code {
            -1 => Err(SzError::unknown(error_msg)),
            -2 => Err(SzError::configuration(error_msg)),
            -3 => Err(SzError::bad_input(error_msg)),
            -4 => Err(SzError::retryable(error_msg)),
            -5 => Err(SzError::unrecoverable(error_msg)),
            -6 => Err(SzError::not_found(error_msg)),
            -7 => Err(SzError::license(error_msg)),
            -8 => Err(SzError::database(error_msg)),
            _ => Err(SzError::unknown(error_msg)),
        }
    }
}

/// Checks the return code from Senzing Config FFI functions
pub fn check_config_return_code(return_code: i64) -> SzResult<()> {
    if return_code == 0 {
        Ok(())
    } else {
        let mut buffer: Vec<u8> = vec![0; 1024];
        let buffer_len = buffer.len() as i64;

        let error_msg = unsafe {
            let actual_len = super::bindings::SzConfig_getLastException(
                buffer.as_mut_ptr() as *mut i8,
                buffer_len,
            );

            if actual_len > 0 && actual_len < buffer_len {
                let string_len = if actual_len > 0 {
                    (actual_len as usize) - 1
                } else {
                    0
                };
                buffer.truncate(string_len);
                String::from_utf8_lossy(&buffer).to_string()
            } else {
                format!("Unknown Config error (code: {})", return_code)
            }
        };

        // Create error with proper Senzing message - backtrace is captured at error creation
        match return_code {
            -1 => Err(SzError::unknown(error_msg)),
            -2 => Err(SzError::configuration(error_msg)),
            -3 => Err(SzError::bad_input(error_msg)),
            -4 => Err(SzError::retryable(error_msg)),
            -5 => Err(SzError::unrecoverable(error_msg)),
            -6 => Err(SzError::not_found(error_msg)),
            -7 => Err(SzError::license(error_msg)),
            -8 => Err(SzError::database(error_msg)),
            _ => Err(SzError::unknown(error_msg)),
        }
    }
}

/// Checks the return code from Senzing ConfigMgr FFI functions
pub fn check_config_mgr_return_code(return_code: i64) -> SzResult<()> {
    if return_code == 0 {
        Ok(())
    } else {
        let mut buffer: Vec<u8> = vec![0; 1024];
        let buffer_len = buffer.len() as i64;

        let error_msg = unsafe {
            let actual_len = super::bindings::SzConfigMgr_getLastException(
                buffer.as_mut_ptr() as *mut i8,
                buffer_len,
            );

            if actual_len > 0 && actual_len < buffer_len {
                let string_len = if actual_len > 0 {
                    (actual_len as usize) - 1
                } else {
                    0
                };
                buffer.truncate(string_len);
                String::from_utf8_lossy(&buffer).to_string()
            } else {
                format!("Unknown ConfigMgr error (code: {})", return_code)
            }
        };

        // Create error with proper Senzing message - backtrace is now captured automatically
        match return_code {
            -1 => Err(SzError::unknown(error_msg)),
            -2 => Err(SzError::configuration(error_msg)),
            -3 => Err(SzError::bad_input(error_msg)),
            -4 => Err(SzError::retryable(error_msg)),
            -5 => Err(SzError::unrecoverable(error_msg)),
            -6 => Err(SzError::not_found(error_msg)),
            -7 => Err(SzError::license(error_msg)),
            -8 => Err(SzError::database(error_msg)),
            _ => Err(SzError::unknown(error_msg)),
        }
    }
}

/// Checks the return code from SzProduct FFI functions
pub fn check_product_return_code(return_code: i64) -> SzResult<()> {
    if return_code == 0 {
        Ok(())
    } else {
        let mut buffer: Vec<u8> = vec![0; 1024];
        let buffer_len = buffer.len() as i64;

        let error_msg = unsafe {
            let actual_len = super::bindings::SzProduct_getLastException(
                buffer.as_mut_ptr() as *mut i8,
                buffer_len,
            );

            if actual_len > 0 && actual_len < buffer_len {
                let string_len = if actual_len > 0 {
                    (actual_len as usize) - 1
                } else {
                    0
                };
                buffer.truncate(string_len);
                String::from_utf8_lossy(&buffer).to_string()
            } else {
                format!("Unknown error (code: {})", return_code)
            }
        };

        // Create error with proper Senzing message
        match return_code {
            -1 => Err(SzError::unknown(error_msg)),
            -2 => Err(SzError::configuration(error_msg)),
            -3 => Err(SzError::bad_input(error_msg)),
            -4 => Err(SzError::retryable(error_msg)),
            -5 => Err(SzError::unrecoverable(error_msg)),
            -6 => Err(SzError::not_found(error_msg)),
            -7 => Err(SzError::license(error_msg)),
            -8 => Err(SzError::database(error_msg)),
            _ => Err(SzError::unknown(error_msg)),
        }
    }
}

/// Checks the return code from Senzing FFI functions (deprecated alias)
#[deprecated(note = "Use check_return_code directly")]
pub fn check_return_code_i64(return_code: i64) -> SzResult<()> {
    check_return_code(return_code)
}

/// Macro for safely calling FFI functions with proper error handling
#[macro_export]
macro_rules! ffi_call {
    ($ffi_fn:expr) => {{
        #[allow(clippy::macro_metavars_in_unsafe)]
        let result = unsafe { $ffi_fn };
        $crate::ffi::helpers::check_return_code(result)?;
    }};
}

/// Macro for safely calling Config FFI functions with proper error handling
#[macro_export]
macro_rules! ffi_call_config {
    ($ffi_fn:expr) => {{
        #[allow(clippy::macro_metavars_in_unsafe)]
        let result = unsafe { $ffi_fn };
        $crate::ffi::helpers::check_config_return_code(result)?;
    }};
}

/// Macro for safely calling ConfigMgr FFI functions with proper error handling
#[macro_export]
macro_rules! ffi_call_config_mgr {
    ($ffi_fn:expr) => {{
        #[allow(clippy::macro_metavars_in_unsafe)]
        let result = unsafe { $ffi_fn };
        $crate::ffi::helpers::check_config_mgr_return_code(result)?;
    }};
}

/// Macro for safely calling Product FFI functions with proper error handling
#[macro_export]
macro_rules! ffi_call_product {
    ($ffi_fn:expr) => {{
        #[allow(clippy::macro_metavars_in_unsafe)]
        let result = unsafe { $ffi_fn };
        $crate::ffi::helpers::check_product_return_code(result)?;
    }};
}

/// Macro for safely calling FFI functions that return i64
#[macro_export]
macro_rules! ffi_call_i64 {
    ($ffi_fn:expr) => {{
        #[allow(clippy::macro_metavars_in_unsafe)]
        let result = unsafe { $ffi_fn };
        $crate::ffi::helpers::check_return_code(result)?;
    }};
}

/// Macro for FFI calls that return response data
#[macro_export]
macro_rules! ffi_call_with_response {
    ($ffi_fn:expr) => {{
        let mut response = $crate::ffi::helpers::ResponseBuffer::new();
        let result = unsafe { $ffi_fn(&mut response.ptr, &mut response.size) };
        $crate::ffi::helpers::check_return_code(result)?;
        response.as_string()
    }};
}
