//! Core LLVM functionality and context management
//!
//! This module provides the fundamental LLVM context, module management,
//! and basic LLVM operations.

use crate::error::{Error, Result};
use std::ptr;

/// LLVM Context wrapper for safe memory management
pub struct Context {
    _context: ptr::NonNull<llvm_sys::LLVMContext>,
}

impl Context {
    /// Create a new LLVM context
    pub fn new() -> Result<Self> {
        let context = unsafe {
            let ctx = llvm_sys::core::LLVMContextCreate();
            ptr::NonNull::new(ctx).ok_or_else(|| Error::InitializationFailed("Failed to create LLVM context".to_string()))?
        };

        Ok(Context { _context: context })
    }

    /// Get the underlying LLVM context pointer
    pub fn as_ptr(&self) -> *mut llvm_sys::LLVMContext {
        self._context.as_ptr()
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            llvm_sys::core::LLVMContextDispose(self._context.as_ptr());
        }
    }
}

/// LLVM Module wrapper for safe module management
pub struct Module {
    _module: ptr::NonNull<llvm_sys::LLVMModule>,
    _context: ptr::NonNull<llvm_sys::LLVMContext>,
}

impl Module {
    /// Create a new LLVM module
    pub fn new(name: &str, context: &Context) -> Result<Self> {
        let module = unsafe {
            let name_cstr = std::ffi::CString::new(name).map_err(|_| Error::ModuleCreationFailed("Invalid module name".to_string()))?;
            let module = llvm_sys::core::LLVMModuleCreateWithNameInContext(name_cstr.as_ptr(), context.as_ptr());
            ptr::NonNull::new(module).ok_or_else(|| Error::ModuleCreationFailed("Failed to create LLVM module".to_string()))?
        };

        Ok(Module {
            _module: module,
            _context: ptr::NonNull::new(context.as_ptr()).unwrap(),
        })
    }

    /// Get the module name
    pub fn get_name(&self) -> &str {
        unsafe {
            let name_ptr = llvm_sys::core::LLVMGetModuleIdentifier(self._module.as_ptr(), ptr::null_mut());
            if name_ptr.is_null() {
                ""
            } else {
                std::ffi::CStr::from_ptr(name_ptr).to_str().unwrap_or("")
            }
        }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        // LLVM modules are typically owned by the context, so we don't dispose them here
        // The context disposal will handle module cleanup
    }
}
