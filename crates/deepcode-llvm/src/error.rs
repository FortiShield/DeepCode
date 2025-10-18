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
