//! Utility functions and helper types for LLVM bindings
//!
//! This module provides common utilities and helper functions that are
//! used throughout the LLVM bindings.

use std::ffi;

/// Helper function to create a CString safely
pub fn cstring_from_str(s: &str) -> Result<ffi::CString, crate::error::Error> {
    ffi::CString::new(s).map_err(|_| crate::error::Error::IrGenerationFailed(
        format!("Invalid string for C FFI: {}", s)
    ))
}

/// Helper function to convert LLVM boolean to Rust bool
pub fn llvm_bool_to_bool(value: i32) -> bool {
    value != 0
}

/// Helper function to convert Rust bool to LLVM boolean
pub fn bool_to_llvm_bool(value: bool) -> i32 {
    if value { 1 } else { 0 }
}

/// Version information for LLVM
pub struct LlvmVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl LlvmVersion {
    /// Get the current LLVM version
    pub fn current() -> Self {
        unsafe {
            let mut major: u32 = 0;
            let mut minor: u32 = 0;
            let mut patch: u32 = 0;

            llvm_sys::core::LLVMGetVersion(&mut major, &mut minor, &mut patch);

            LlvmVersion { major, minor, patch }
        }
    }

    /// Get version as a string
    pub fn as_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Initialize LLVM subsystems
pub fn initialize_llvm() -> Result<(), crate::error::Error> {
    unsafe {
        llvm_sys::target::LLVM_InitializeAllTargetInfos();
        llvm_sys::target::LLVM_InitializeAllTargets();
        llvm_sys::target::LLVM_InitializeAllTargetMCs();
        llvm_sys::target::LLVM_InitializeAllAsmPrinters();
        llvm_sys::target::LLVM_InitializeAllAsmParsers();
        llvm_sys::analysis::LLVMInitializeCoreAnalysis();
        llvm_sys::transforms::scalar::LLVMInitializeScalarOpts();
    }
    Ok(())
}
