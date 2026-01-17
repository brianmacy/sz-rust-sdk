//! Helper functions for FFI operations

use crate::error::{SzError, SzResult};
use libc::{c_char, c_void, size_t};
use std::ffi::{CStr, CString};
use std::ptr;

/// Converts a Rust string to a C string (Internal)
pub(crate) fn str_to_c_string(s: &str) -> SzResult<CString> {
    CString::new(s).map_err(SzError::from)
}

/// Frees memory allocated by Senzing helper functions
///
/// # Safety
/// ptr must be a valid pointer allocated by Senzing or null
#[inline]
pub(crate) unsafe fn sz_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe { super::SzHelper_free(ptr as *mut c_void) };
    }
}

/// Converts a C string pointer to a Rust string and frees the C string
///
/// # Safety
///
/// The caller must ensure that `ptr` is either null or a valid pointer to a null-terminated C string
/// that was allocated by the Senzing library.
pub(crate) unsafe fn c_str_to_string(ptr: *mut c_char) -> SzResult<String> {
    if ptr.is_null() {
        return Ok(String::new());
    }

    let c_str = unsafe { CStr::from_ptr(ptr) };
    let result = match c_str.to_str() {
        Ok(s) => Ok(s.to_string()),
        Err(_) => {
            // If the C string contains invalid UTF-8, convert it to hex encoding to preserve binary data
            let bytes = c_str.to_bytes();
            Ok(hex::encode(bytes))
        }
    };

    // Free the C string memory using Senzing's free function
    unsafe { sz_free(ptr) };

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
            let bytes = c_str.to_bytes();
            Ok(hex::encode(bytes))
        }
    }
}

/// Converts C string to raw bytes for handle storage
///
/// # Safety
///
/// The caller must ensure that ptr is either null or points to a valid
/// null-terminated C string that can be safely freed.
pub(crate) unsafe fn c_str_to_bytes(ptr: *mut c_char) -> SzResult<Vec<u8>> {
    if ptr.is_null() {
        return Ok(Vec::new());
    }

    let c_str = unsafe { CStr::from_ptr(ptr) };
    let bytes = c_str.to_bytes().to_vec();

    unsafe { sz_free(ptr) };

    Ok(bytes)
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
                sz_free(self.ptr);
            }
        }
    }
}

/// Checks the return code from Senzing Engine FFI functions
pub(crate) fn check_return_code(return_code: i64) -> SzResult<()> {
    if return_code == 0 {
        return Ok(());
    }

    let actual_error_code = unsafe { super::Sz_getLastExceptionCode() };
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

    let actual_error_code = unsafe { super::SzConfig_getLastExceptionCode() };
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

    let actual_error_code = unsafe { super::SzConfigMgr_getLastExceptionCode() };
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

    let actual_error_code = unsafe { super::SzProduct_getLastExceptionCode() };
    Err(SzError::from_code_with_message(
        actual_error_code,
        crate::error::SzComponent::Product,
    ))
}

/// Checks the return code from SzDiagnostic FFI functions
pub(crate) fn check_diagnostic_return_code(return_code: i64) -> SzResult<()> {
    if return_code == 0 {
        return Ok(());
    }

    let actual_error_code = unsafe { super::SzDiagnostic_getLastExceptionCode() };
    Err(SzError::from_code_with_message(
        actual_error_code,
        crate::error::SzComponent::Diagnostic,
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

/// Macro for safely calling Diagnostic FFI functions with proper error handling
#[doc(hidden)]
#[macro_export]
macro_rules! ffi_call_diagnostic {
    ($ffi_fn:expr) => {{
        #[allow(clippy::macro_metavars_in_unsafe)]
        let result = unsafe { $ffi_fn };
        $crate::ffi::helpers::check_diagnostic_return_code(result)?;
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

/// Macro to process helper function results that have response and returnCode fields
/// Works with any bindgen-generated result type
#[doc(hidden)]
#[macro_export]
macro_rules! process_result {
    ($result:expr, $check_fn:path) => {{
        $check_fn($result.returnCode)?;
        unsafe { $crate::ffi::helpers::c_str_to_string($result.response) }
    }};
}

/// Process engine helper result
#[doc(hidden)]
#[macro_export]
macro_rules! process_engine_result {
    ($result:expr) => {{
        $crate::ffi::helpers::check_return_code($result.returnCode)?;
        unsafe { $crate::ffi::helpers::c_str_to_string($result.response) }
    }};
}

/// Process config helper result
#[doc(hidden)]
#[macro_export]
macro_rules! process_config_result {
    ($result:expr) => {{
        $crate::ffi::helpers::check_config_return_code($result.returnCode)?;
        unsafe { $crate::ffi::helpers::c_str_to_string($result.response) }
    }};
}

/// Process config manager helper result (string response)
#[doc(hidden)]
#[macro_export]
macro_rules! process_config_mgr_result {
    ($result:expr) => {{
        $crate::ffi::helpers::check_config_mgr_return_code($result.returnCode)?;
        unsafe { $crate::ffi::helpers::c_str_to_string($result.response) }
    }};
}

/// Process config manager helper result (i64 response like configID)
#[doc(hidden)]
#[macro_export]
macro_rules! process_config_mgr_long_result {
    ($result:expr) => {{
        $crate::ffi::helpers::check_config_mgr_return_code($result.returnCode)?;
        Ok($result.configID)
    }};
}

/// Process diagnostic helper result
#[doc(hidden)]
#[macro_export]
macro_rules! process_diagnostic_result {
    ($result:expr) => {{
        $crate::ffi::helpers::check_diagnostic_return_code($result.returnCode)?;
        unsafe { $crate::ffi::helpers::c_str_to_string($result.response) }
    }};
}

/// Process product helper result
#[doc(hidden)]
#[macro_export]
macro_rules! process_product_result {
    ($result:expr) => {{
        $crate::ffi::helpers::check_product_return_code($result.returnCode)?;
        unsafe { $crate::ffi::helpers::c_str_to_string($result.response) }
    }};
}
