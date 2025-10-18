//! # DeepCode LLVM Bindings
//!
//! Rust bindings for LLVM libraries used by CubeCL for GPU code generation and optimization.
//!
//! This crate provides safe Rust interfaces to LLVM's C API, enabling:
//! - LLVM IR generation and manipulation
//! - Code optimization passes
//! - Target code generation for various architectures
//! - Integration with MLIR for high-level compiler infrastructure

// Re-export deepcode-mlir-rs under `mlir_rs`
pub mod mlir_rs {
    pub use deepcode_mlir_rs::*;
}

// Re-export deepcode-mlir-sys under `mlir_sys`
pub mod mlir_sys {
    pub use deepcode_mlir_sys::*;
}

// Core LLVM bindings modules
pub mod core;
pub mod ir;
pub mod target;
pub mod analysis;
pub mod transforms;

// Utility modules
pub mod utils;
pub mod error;

// Prelude for convenient imports
pub mod prelude {
    pub use super::core::*;
    pub use super::ir::*;
    pub use super::target::*;
    pub use super::error::{Result, Error};
}

// Re-export commonly used types for convenience
pub use error::{Result, Error};
