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

// Unit tests for utility functions

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cstring_from_str_valid() {
        let result = cstring_from_str("valid_string");
        assert!(result.is_ok());

        let cstring = result.unwrap();
        assert_eq!(cstring.to_str().unwrap(), "valid_string");
    }

    #[test]
    fn test_cstring_from_str_invalid() {
        // CString::new fails on strings containing null bytes
        let result = cstring_from_str("invalid\0string");
        assert!(result.is_err());

        match result.unwrap_err() {
            crate::error::Error::IrGenerationFailed(msg) => {
                assert!(msg.contains("Invalid string for C FFI"));
            }
            _ => panic!("Expected IrGenerationFailed error"),
        }
    }

    #[test]
    fn test_cstring_from_str_empty() {
        let result = cstring_from_str("");
        assert!(result.is_ok());

        let cstring = result.unwrap();
        assert_eq!(cstring.to_str().unwrap(), "");
    }

    #[test]
    fn test_llvm_bool_to_bool_true() {
        assert_eq!(llvm_bool_to_bool(1), true);
        assert_eq!(llvm_bool_to_bool(42), true); // Any non-zero value
        assert_eq!(llvm_bool_to_bool(-1), true); // Negative values
    }

    #[test]
    fn test_llvm_bool_to_bool_false() {
        assert_eq!(llvm_bool_to_bool(0), false);
    }

    #[test]
    fn test_bool_to_llvm_bool_true() {
        assert_eq!(bool_to_llvm_bool(true), 1);
    }

    #[test]
    fn test_bool_to_llvm_bool_false() {
        assert_eq!(bool_to_llvm_bool(false), 0);
    }

    #[test]
    fn test_llvm_version_structure() {
        let version = LlvmVersion {
            major: 20,
            minor: 1,
            patch: 4,
        };

        assert_eq!(version.major, 20);
        assert_eq!(version.minor, 1);
        assert_eq!(version.patch, 4);
    }

    #[test]
    fn test_llvm_version_as_string() {
        let version = LlvmVersion {
            major: 20,
            minor: 1,
            patch: 4,
        };

        assert_eq!(version.as_string(), "20.1.4");
    }

    #[test]
    fn test_llvm_version_zero() {
        let version = LlvmVersion {
            major: 0,
            minor: 0,
            patch: 0,
        };

        assert_eq!(version.as_string(), "0.0.0");
    }

    #[test]
    fn test_initialize_llvm() {
        // This test mainly checks that the function doesn't panic
        // In a real environment with LLVM installed, this would initialize LLVM
        let result = initialize_llvm();
        // The result depends on whether LLVM is actually available
        // For testing purposes, we'll just check it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_bool_conversion_roundtrip() {
        // Test that bool -> llvm_bool -> bool conversion works correctly
        assert_eq!(llvm_bool_to_bool(bool_to_llvm_bool(true)), true);
        assert_eq!(llvm_bool_to_bool(bool_to_llvm_bool(false)), false);
    }

    #[test]
    fn test_cstring_edge_cases() {
        // Test various edge cases for CString creation
        assert!(cstring_from_str("normal_string").is_ok());
        assert!(cstring_from_str("string_with_spaces").is_ok());
        assert!(cstring_from_str("string-with-dashes").is_ok());
        assert!(cstring_from_str("string_with_underscores").is_ok());

        // Unicode strings should work
        assert!(cstring_from_str("café").is_ok());
        assert!(cstring_from_str("日本語").is_ok());
    }
}
