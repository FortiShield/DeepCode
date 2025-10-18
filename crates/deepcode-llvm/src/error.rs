//! Error handling for LLVM bindings
//!
//! This module provides error types and utilities for handling LLVM-related errors
//! in a safe and ergonomic way.

use std::fmt;

/// Result type alias for LLVM operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for LLVM bindings
#[derive(Debug)]
pub enum Error {
    /// LLVM initialization failed
    InitializationFailed(String),
    /// Invalid LLVM context
    InvalidContext,
    /// Module creation failed
    ModuleCreationFailed(String),
    /// IR generation error
    IrGenerationFailed(String),
    /// Target machine creation failed
    TargetMachineFailed(String),
    /// Code generation failed
    CodeGenerationFailed(String),
    /// MLIR integration error
    MlirError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InitializationFailed(msg) => write!(f, "LLVM initialization failed: {}", msg),
            Error::InvalidContext => write!(f, "Invalid LLVM context"),
            Error::ModuleCreationFailed(msg) => write!(f, "Module creation failed: {}", msg),
            Error::IrGenerationFailed(msg) => write!(f, "IR generation failed: {}", msg),
            Error::TargetMachineFailed(msg) => write!(f, "Target machine creation failed: {}", msg),
            Error::CodeGenerationFailed(msg) => write!(f, "Code generation failed: {}", msg),
            Error::MlirError(msg) => write!(f, "MLIR integration error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::IrGenerationFailed(msg)
    }
}

// Unit tests for error handling

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_initialization_failed() {
        let error = Error::InitializationFailed("Test message".to_string());
        assert_eq!(error.to_string(), "LLVM initialization failed: Test message");
    }

    #[test]
    fn test_error_display_invalid_context() {
        let error = Error::InvalidContext;
        assert_eq!(error.to_string(), "Invalid LLVM context");
    }

    #[test]
    fn test_error_display_module_creation_failed() {
        let error = Error::ModuleCreationFailed("Module error".to_string());
        assert_eq!(error.to_string(), "Module creation failed: Module error");
    }

    #[test]
    fn test_error_display_ir_generation_failed() {
        let error = Error::IrGenerationFailed("IR error".to_string());
        assert_eq!(error.to_string(), "IR generation failed: IR error");
    }

    #[test]
    fn test_error_display_target_machine_failed() {
        let error = Error::TargetMachineFailed("Target error".to_string());
        assert_eq!(error.to_string(), "Target machine creation failed: Target error");
    }

    #[test]
    fn test_error_display_code_generation_failed() {
        let error = Error::CodeGenerationFailed("Codegen error".to_string());
        assert_eq!(error.to_string(), "Code generation failed: Codegen error");
    }

    #[test]
    fn test_error_display_mlir_error() {
        let error = Error::MlirError("MLIR integration error".to_string());
        assert_eq!(error.to_string(), "MLIR integration error: MLIR integration error");
    }

    #[test]
    fn test_error_debug() {
        let error = Error::InvalidContext;
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("InvalidContext"));
    }

    #[test]
    fn test_error_from_string() {
        let error: Error = "test message".to_string().into();
        match error {
            Error::IrGenerationFailed(msg) => assert_eq!(msg, "test message"),
            _ => panic!("Expected IrGenerationFailed error"),
        }
    }

    #[test]
    fn test_result_type_alias() {
        fn returns_result() -> Result<i32> {
            Ok(42)
        }

        let result = returns_result();
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_result_error() {
        fn returns_error() -> Result<i32> {
            Err(Error::InvalidContext)
        }

        let result = returns_error();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::InvalidContext));
    }

    #[test]
    fn test_error_as_std_error() {
        let error = Error::InitializationFailed("test".to_string());
        let std_error: &dyn std::error::Error = &error;
        assert_eq!(std_error.to_string(), "LLVM initialization failed: test");
    }
}
