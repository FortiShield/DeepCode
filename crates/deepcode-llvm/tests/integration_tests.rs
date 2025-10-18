//! Integration tests for the deepcode-llvm crate
//!
//! These tests verify the overall functionality and integration
//! of the LLVM bindings with the rest of the ecosystem.

use deepcode_llvm::{
    core::{Context, Module},
    error::Result,
    utils::{initialize_llvm, LlvmVersion},
    prelude::*,
};

#[test]
fn test_full_llvm_workflow() -> Result<()> {
    // Initialize LLVM subsystems
    initialize_llvm()?;

    // Create LLVM context
    let context = Context::new()?;

    // Create a test module
    let module = Module::new("integration_test", &context)?;
    assert_eq!(module.get_name(), "integration_test");

    // Verify module was created successfully
    assert!(!module.get_name().is_empty());

    Ok(())
}

#[test]
fn test_llvm_version_detection() {
    let version = LlvmVersion::current();

    // Version should be populated (even if LLVM not available, should not panic)
    assert!(version.major >= 0);
    assert!(version.minor >= 0);
    assert!(version.patch >= 0);

    // Version string should be properly formatted
    let version_str = version.as_string();
    assert!(!version_str.is_empty());
    assert!(version_str.contains('.'));
}

#[test]
fn test_module_with_different_names() -> Result<()> {
    let context = Context::new()?;

    // Test various module name formats
    let test_cases = vec![
        "simple_module",
        "module_with_underscores",
        "module-with-dashes",
        "ModuleWithCamelCase",
        "module123",
        "test-module_123",
    ];

    for name in test_cases {
        let module = Module::new(name, &context)?;
        assert_eq!(module.get_name(), name);
    }

    Ok(())
}

#[test]
fn test_context_module_relationship() -> Result<()> {
    let context = Context::new()?;

    // Create multiple modules with the same context
    let modules: Vec<_> = (0..5)
        .map(|i| Module::new(&format!("module_{}", i), &context))
        .collect::<Result<Vec<_>>>()?;

    // All modules should be valid and have correct names
    for (i, module) in modules.iter().enumerate() {
        assert_eq!(module.get_name(), format!("module_{}", i));
    }

    // All modules should share the same context
    // (In a real implementation, we could verify this by checking internal state)

    Ok(())
}

#[test]
fn test_error_handling_integration() {
    use deepcode_llvm::error::Error;

    // Test that error types work correctly across module boundaries
    let error = Error::InitializationFailed("test error".to_string());

    // Error should implement Display
    let error_msg = error.to_string();
    assert!(error_msg.contains("LLVM initialization failed"));
    assert!(error_msg.contains("test error"));

    // Error should implement Debug
    let debug_msg = format!("{:?}", error);
    assert!(debug_msg.contains("InitializationFailed"));

    // Error should be Send + Sync for cross-thread usage
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Error>();
}

#[test]
fn test_prelude_imports() {
    // Test that the prelude module works correctly
    use deepcode_llvm::prelude::*;

    // These should compile without errors if prelude is working
    let _context = Context::new().expect("Failed to create context");
    let _module = Module::new("prelude_test", &_context).expect("Failed to create module");
}

#[test]
fn test_crate_structure_integrity() {
    // Test that all public modules are accessible
    use deepcode_llvm::{core, ir, target, analysis, transforms, utils, error};

    // These should compile without errors if module structure is correct
    let _context = core::Context::new().expect("Failed to create context");
    let _module = core::Module::new("structure_test", &_context).expect("Failed to create module");

    // Test utility functions
    let version = utils::LlvmVersion::current();
    assert!(version.as_string().len() > 0);

    // Test error types
    let _error = error::Error::InvalidContext;
}

#[test]
fn test_memory_safety_patterns() -> Result<()> {
    // Test that our memory management patterns work correctly
    let context = Context::new()?;

    {
        // Module should be valid within this scope
        let module = Module::new("memory_test", &context)?;
        assert_eq!(module.get_name(), "memory_test");

        // Module should still be accessible
        assert!(!module.get_name().is_empty());
    } // Module dropped here

    // Context should still be valid after module is dropped
    let module2 = Module::new("memory_test2", &context)?;
    assert_eq!(module2.get_name(), "memory_test2");

    Ok(())
}

#[test]
fn test_cross_module_functionality() -> Result<()> {
    use deepcode_llvm::utils;

    // Test that utilities work with core functionality
    let context = Context::new()?;
    let module = Module::new("cross_test", &context)?;

    // Initialize LLVM (utility function)
    utils::initialize_llvm()?;

    // Get LLVM version (utility function)
    let version = utils::LlvmVersion::current();
    assert!(version.as_string().len() > 0);

    // Module should still be valid after utility calls
    assert_eq!(module.get_name(), "cross_test");

    Ok(())
}
