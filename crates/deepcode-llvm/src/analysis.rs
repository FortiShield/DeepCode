//! LLVM analysis and optimization passes
//!
//! This module provides access to LLVM's analysis framework for
//! code analysis and optimization pass management.

use crate::core::Module;
use crate::error::{Error, Result};

/// Analysis pass manager for LLVM modules
pub struct PassManager {
    _pass_manager: *mut llvm_sys::analysis::LLVMPassManager,
}

impl PassManager {
    /// Create a new function pass manager
    pub fn new() -> Result<Self> {
        unsafe {
            let pass_manager = llvm_sys::analysis::LLVMCreatePassManager();

            if pass_manager.is_null() {
                return Err(Error::InitializationFailed("Failed to create pass manager".to_string()));
            }

            Ok(PassManager {
                _pass_manager: pass_manager,
            })
        }
    }

    /// Add basic alias analysis pass
    pub fn add_basic_alias_analysis(&self) -> Result<()> {
        unsafe {
            llvm_sys::analysis::LLVMAddBasicAliasAnalysisPass(self._pass_manager);
            Ok(())
        }
    }

    /// Add type-based alias analysis pass
    pub fn add_type_based_alias_analysis(&self) -> Result<()> {
        unsafe {
            llvm_sys::analysis::LLVMAddTypeBasedAliasAnalysisPass(self._pass_manager);
            Ok(())
        }
    }

    /// Run the pass manager on a module
    pub fn run(&self, module: &Module) -> Result<bool> {
        unsafe {
            let changed = llvm_sys::analysis::LLVMRunPassManager(self._pass_manager, module._module.as_ptr());

            // LLVM returns 1 if changed, 0 if not changed
            Ok(changed != 0)
        }
    }
}

impl Default for PassManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default pass manager")
    }
}

impl Drop for PassManager {
    fn drop(&mut self) {
        unsafe {
            if !self._pass_manager.is_null() {
                llvm_sys::analysis::LLVMDisposePassManager(self._pass_manager);
            }
        }
    }
}
