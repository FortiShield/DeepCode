//! LLVM IR generation and manipulation
//!
//! This module provides functionality for creating, manipulating, and
//! optimizing LLVM Intermediate Representation (IR).

use crate::core::{Context, Module};
use crate::error::{Error, Result};

/// Builder for creating LLVM IR functions and basic blocks
pub struct IrBuilder {
    _context: *mut llvm_sys::LLVMContext,
    _module: *mut llvm_sys::LLVMModule,
}

impl IrBuilder {
    /// Create a new IR builder
    pub fn new(module: &Module) -> Self {
        IrBuilder {
            _context: module._context.as_ptr(),
            _module: module._module.as_ptr(),
        }
    }

    /// Create a simple test function for verification
    pub fn create_test_function(&self) -> Result<()> {
        unsafe {
            // Create function type (void -> void)
            let void_type = llvm_sys::core::LLVMVoidTypeInContext(self._context);
            let function_type = llvm_sys::core::LLVMFunctionType(void_type, ptr::null_mut(), 0, 0);

            // Create the function
            let function_name = std::ffi::CString::new("test_function")
                .map_err(|_| Error::IrGenerationFailed("Invalid function name".to_string()))?;

            let function = llvm_sys::core::LLVMAddFunction(self._module, function_name.as_ptr(), function_type);

            if function.is_null() {
                return Err(Error::IrGenerationFailed("Failed to create function".to_string()));
            }

            // Create basic block
            let basic_block = llvm_sys::core::LLVMAppendBasicBlockInContext(
                self._context,
                function,
                std::ffi::CString::new("entry")
                    .map_err(|_| Error::IrGenerationFailed("Invalid block name".to_string()))?
                    .as_ptr(),
            );

            if basic_block.is_null() {
                return Err(Error::IrGenerationFailed("Failed to create basic block".to_string()));
            }

            Ok(())
        }
    }
}

/// Verify that an LLVM module is well-formed
pub fn verify_module(module: &Module) -> Result<()> {
    unsafe {
        let mut error_message: *mut i8 = ptr::null_mut();
        let result = llvm_sys::analysis::LLVMVerifyModule(
            module._module.as_ptr(),
            llvm_sys::analysis::LLVMVerifierFailureAction::LLVMReturnStatusAction,
            &mut error_message as *mut *mut i8,
        );

        if result != 0 {
            let error_msg = if error_message.is_null() {
                "Module verification failed".to_string()
            } else {
                let cstr = std::ffi::CStr::from_ptr(error_message);
                cstr.to_string_lossy().to_string()
            };

            if !error_message.is_null() {
                libc::free(error_message as *mut libc::c_void);
            }

            return Err(Error::IrGenerationFailed(error_msg));
        }

        Ok(())
    }
}
