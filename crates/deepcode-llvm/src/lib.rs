//! # DeepCode LLVM Bindings
//!
//! Rust bindings for LLVM libraries used by CubeCL for GPU code generation and optimization.
//!
//! This crate provides safe Rust interfaces to LLVM's C API, enabling:
//! - LLVM IR generation and manipulation
//! - Code optimization passes
//! - Target code generation for various architectures
//! - Integration with MLIR for high-level compiler infrastructure
//!
//! ## Basic Usage
//!
//! ```rust,no_run
//! use deepcode_llvm::{core::{Context, Module}, utils::initialize_llvm};
//!
//! // Initialize LLVM subsystems
//! initialize_llvm().expect("Failed to initialize LLVM");
//!
//! // Create LLVM context and module
//! let context = Context::new().expect("Failed to create context");
//! let module = Module::new("example", &context).expect("Failed to create module");
//!
//! println!("Created module: {}", module.get_name());
//! ```
//!
//! ## Error Handling
//!
//! ```rust,no_run
//! use deepcode_llvm::{core::Context, error::{Error, Result}};
//!
//! fn example_with_error_handling() -> Result<()> {
//!     let context = Context::new()
//!         .map_err(|e| {
//!             eprintln!("Context creation failed: {}", e);
//!             e
//!         })?;
//!
//!     // Use context...
//!     Ok(())
//! }
//! ```
//!
//! ## Advanced Features
//!
//! ```rust,no_run
//! use deepcode_llvm::{
//!     core::{Context, Module},
//!     target::{TargetMachine, TargetArch},
//!     analysis::PassManager,
//!     prelude::*
//! };
//!
//! // Multi-architecture code generation
//! let target_machine = TargetMachine::new(TargetArch::X86)?;
//! let pass_manager = PassManager::new()?;
//!
//! // Add optimization passes
//! pass_manager.add_basic_alias_analysis()?;
//! pass_manager.add_constant_propagation()?;
//! ```
//!
//! ## Architecture Overview
//!
//! - **Core**: Context and module lifecycle management
//! - **IR**: LLVM IR generation and manipulation
//! - **Target**: Multi-architecture code generation
//! - **Analysis**: Static analysis and optimization passes
//! - **Transforms**: Code optimization and transformation
//! - **Utils**: Helper functions and version management
//! - **Error**: Comprehensive error handling
//! - **Build Verify**: Compile-time verification

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
pub mod build_verify;

// Prelude for convenient imports
pub mod prelude {
    pub use super::core::*;
    pub use super::ir::*;
    pub use super::target::*;
    pub use super::error::{Result, Error};
    pub use super::build_verify::*;
}

// Re-export commonly used types for convenience
pub use error::{Result, Error};
