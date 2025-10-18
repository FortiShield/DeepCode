//! LLVM transformation passes for code optimization
//!
//! This module provides access to LLVM's transformation passes for
//! optimizing generated code.

use crate::core::Module;
use crate::error::{Error, Result};

/// Transformation pass manager for code optimization
pub struct TransformPassManager {
    _pass_manager: *mut llvm_sys::transforms::pass_manager::LLVMPassManager,
}

impl TransformPassManager {
    /// Create a new transformation pass manager
    pub fn new() -> Result<Self> {
        unsafe {
            let pass_manager = llvm_sys::transforms::pass_manager::LLVMCreatePassManager();

            if pass_manager.is_null() {
                return Err(Error::InitializationFailed("Failed to create transform pass manager".to_string()));
            }

            Ok(TransformPassManager {
                _pass_manager: pass_manager,
            })
        }
    }

    /// Add dead code elimination pass
    pub fn add_dead_code_elimination(&self) -> Result<()> {
        unsafe {
            llvm_sys::transforms::scalar::LLVMAddDeadStoreEliminationPass(self._pass_manager);
            Ok(())
        }
    }

    /// Add constant propagation pass
    pub fn add_constant_propagation(&self) -> Result<()> {
        unsafe {
            llvm_sys::transforms::scalar::LLVMAddConstantPropagationPass(self._pass_manager);
            Ok(())
        }
    }

    /// Add instruction combining pass
    pub fn add_instruction_combining(&self) -> Result<()> {
        unsafe {
            llvm_sys::transforms::scalar::LLVMAddInstructionCombiningPass(self._pass_manager);
            Ok(())
        }
    }

    /// Add global value numbering pass
    pub fn add_gvn(&self) -> Result<()> {
        unsafe {
            llvm_sys::transforms::scalar::LLVMAddGVNPass(self._pass_manager);
            Ok(())
        }
    }

    /// Run all transformation passes on a module
    pub fn run(&self, module: &Module) -> Result<bool> {
        unsafe {
            let changed = llvm_sys::transforms::pass_manager::LLVMRunPassManager(
                self._pass_manager,
                module._module.as_ptr(),
            );

            Ok(changed != 0)
        }
    }
}

impl Default for TransformPassManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default transform pass manager")
    }
}

impl Drop for TransformPassManager {
    fn drop(&mut self) {
        unsafe {
            if !self._pass_manager.is_null() {
                llvm_sys::transforms::pass_manager::LLVMDisposePassManager(self._pass_manager);
            }
        }
    }
}
