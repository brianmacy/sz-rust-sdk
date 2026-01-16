//! Helper functions for FFI operations

use crate::error::{SzError, SzResult};
use libc::{c_char, size_t};
use std::ffi::{CStr, CString};
use std::ptr;

/// Converts a Rust string to a C string (Internal)
pub(crate) fn str_to_c_string(s: &str) -> SzResult<CString> {
    CString::new(s).map_err(SzError::from)
}

/// Converts a C string pointer to a Rust string and frees the C string
///
/// # Safety
///
/// The caller must ensure that `ptr` is either null or a valid pointer to a null-terminated C string
/// that was allocated by the Senzing library and can be freed with `Sz_free`.
pub(crate) unsafe fn c_str_to_string(ptr: *mut c_char) -> SzResult<String> {
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
pub(crate) unsafe fn c_str_to_string_no_free(ptr: *mut c_char) -> SzResult<String> {
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
pub(crate) unsafe fn process_pointer_result(
    result: super::bindings::SzPointerResult,
) -> SzResult<String> {
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
pub(crate) unsafe fn process_config_pointer_result(
    result: super::bindings::SzPointerResult,
) -> SzResult<String> {
    check_config_return_code(result.return_code)?;
    unsafe { c_str_to_string(result.response) }
}

/// Processes an SzPointerResult from config helper functions and returns raw bytes
///
/// # Safety
///
/// The caller must ensure that the SzPointerResult contains valid data
/// and that any response pointer is valid.
pub(crate) unsafe fn process_config_pointer_result_bytes(
    result: super::bindings::SzPointerResult,
) -> SzResult<Vec<u8>> {
    check_config_return_code(result.return_code)?;
    unsafe { c_str_to_bytes(result.response) }
}

/// Processes an SzPointerResult from engine helper functions
///
/// # Safety
///
/// The caller must ensure that the SzPointerResult contains valid data
/// and that any response pointer is valid and null-terminated.
pub(crate) unsafe fn process_engine_pointer_result(
    result: super::bindings::SzPointerResult,
) -> SzResult<String> {
    check_return_code(result.return_code)?;
    unsafe { c_str_to_string(result.response) }
}

/// Converts C string to raw bytes for handle storage
///
/// # Safety
///
/// The caller must ensure that ptr is either null or points to a valid
/// null-terminated C string that can be safely freed with Sz_free.
pub(crate) unsafe fn c_str_to_bytes(ptr: *mut c_char) -> SzResult<Vec<u8>> {
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
pub(crate) unsafe fn process_config_mgr_pointer_result(
    result: super::bindings::SzPointerResult,
) -> SzResult<String> {
    check_config_mgr_return_code(result.return_code)?;
    unsafe { c_str_to_string(result.response) }
}

/// Processes an SzLongResult from config manager helper functions
pub(crate) fn process_config_mgr_long_result(
    result: super::bindings::SzLongResult,
) -> SzResult<i64> {
    check_config_mgr_return_code(result.return_code)?;
    Ok(result.response)
}

/// Processes an SzLongResult from helper functions
pub(crate) fn process_long_result(result: super::bindings::SzLongResult) -> SzResult<i64> {
    if result.return_code != 0 {
        return Err(SzError::from_code(result.return_code));
    }

    Ok(result.response)
}

/// Handles memory allocation for FFI response strings
pub(crate) struct ResponseBuffer {
    pub ptr: *mut c_char,
    pub size: size_t,
}

impl Default for ResponseBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl ResponseBuffer {
    pub(crate) fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            size: 0,
        }
    }

    pub(crate) fn as_string(&self) -> SzResult<String> {
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
pub(crate) fn check_return_code(return_code: i64) -> SzResult<()> {
    if return_code == 0 {
        return Ok(());
    }

    // Get the actual Senzing error code from getLastExceptionCode()
    let actual_error_code = unsafe { super::bindings::Sz_getLastExceptionCode() };

    // Use the error module's mapping which uses the actual Senzing error code
    Err(SzError::from_code_with_message(
        actual_error_code,
        crate::error::SzComponent::Engine,
    ))
}

/// Checks the return code from Senzing Config FFI functions
pub(crate) fn check_config_return_code(return_code: i64) -> SzResult<()> {
    if return_code == 0 {
        return Ok(());
    }

    // Get the actual Senzing error code from getLastExceptionCode()
    let actual_error_code = unsafe { super::bindings::SzConfig_getLastExceptionCode() };

    // Use the error module's mapping which uses the actual Senzing error code
    Err(SzError::from_code_with_message(
        actual_error_code,
        crate::error::SzComponent::Config,
    ))
}

/// Checks the return code from Senzing ConfigMgr FFI functions
pub(crate) fn check_config_mgr_return_code(return_code: i64) -> SzResult<()> {
    if return_code == 0 {
        return Ok(());
    }

    // Get the actual Senzing error code from getLastExceptionCode()
    let actual_error_code = unsafe { super::bindings::SzConfigMgr_getLastExceptionCode() };

    // Use the error module's mapping which uses the actual Senzing error code
    Err(SzError::from_code_with_message(
        actual_error_code,
        crate::error::SzComponent::ConfigMgr,
    ))
}

/// Checks the return code from SzProduct FFI functions
pub(crate) fn check_product_return_code(return_code: i64) -> SzResult<()> {
    if return_code == 0 {
        return Ok(());
    }

    // Get the actual Senzing error code from getLastExceptionCode()
    let actual_error_code = unsafe { super::bindings::SzProduct_getLastExceptionCode() };

    // Use the error module's mapping which uses the actual Senzing error code
    Err(SzError::from_code_with_message(
        actual_error_code,
        crate::error::SzComponent::Product,
    ))
}

/// Macro for safely calling FFI functions with proper error handling
#[doc(hidden)]
#[macro_export]
macro_rules! ffi_call {
    ($ffi_fn:expr) => {{
        #[allow(clippy::macro_metavars_in_unsafe)]
        let result = unsafe { $ffi_fn };
        $crate::ffi::helpers::check_return_code(result)?;
    }};
}

/// Macro for safely calling Config FFI functions with proper error handling
#[doc(hidden)]
#[macro_export]
macro_rules! ffi_call_config {
    ($ffi_fn:expr) => {{
        #[allow(clippy::macro_metavars_in_unsafe)]
        let result = unsafe { $ffi_fn };
        $crate::ffi::helpers::check_config_return_code(result)?;
    }};
}

/// Macro for safely calling ConfigMgr FFI functions with proper error handling
#[doc(hidden)]
#[macro_export]
macro_rules! ffi_call_config_mgr {
    ($ffi_fn:expr) => {{
        #[allow(clippy::macro_metavars_in_unsafe)]
        let result = unsafe { $ffi_fn };
        $crate::ffi::helpers::check_config_mgr_return_code(result)?;
    }};
}

/// Macro for safely calling Product FFI functions with proper error handling
#[doc(hidden)]
#[macro_export]
macro_rules! ffi_call_product {
    ($ffi_fn:expr) => {{
        #[allow(clippy::macro_metavars_in_unsafe)]
        let result = unsafe { $ffi_fn };
        $crate::ffi::helpers::check_product_return_code(result)?;
    }};
}

/// Macro for safely calling FFI functions that return i64
#[doc(hidden)]
#[macro_export]
macro_rules! ffi_call_i64 {
    ($ffi_fn:expr) => {{
        #[allow(clippy::macro_metavars_in_unsafe)]
        let result = unsafe { $ffi_fn };
        $crate::ffi::helpers::check_return_code(result)?;
    }};
}

/// Macro for FFI calls that return response data
#[doc(hidden)]
#[macro_export]
macro_rules! ffi_call_with_response {
    ($ffi_fn:expr) => {{
        let mut response = $crate::ffi::helpers::ResponseBuffer::new();
        let result = unsafe { $ffi_fn(&mut response.ptr, &mut response.size) };
        $crate::ffi::helpers::check_return_code(result)?;
        response.as_string()
    }};
}
