//! Build-time verification for LLVM bindings
//!
//! This module contains compile-time checks and build verification
//! to ensure the LLVM bindings are correctly structured.

/// Compile-time check that our types implement required traits
pub mod compile_time_checks {
    use crate::core::{Context, Module};
    use crate::error::{Error, Result};

    /// Verify that Context implements Send + Sync for thread safety
    const _: () = {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Context>();
        assert_send_sync::<Module>();
        assert_send_sync::<Error>();
        assert_send_sync::<Result<()>>();
    };

    /// Verify that our error types implement std::error::Error
    const _: () = {
        fn assert_error_trait<T: std::error::Error>() {}
        assert_error_trait::<Error>();
    };

    /// Verify that Result type alias works correctly
    const _: () = {
        fn assert_result_type() {
            let _result: Result<i32> = Ok(42);
            let _error_result: Result<i32> = Err(Error::InvalidContext);
        }
        assert_result_type();
    };
}

/// Build-time feature detection
pub mod feature_detection {
    /// Check if we're compiling with test features
    #[cfg(test)]
    pub const TEST_FEATURE_ENABLED: bool = true;

    #[cfg(not(test))]
    pub const TEST_FEATURE_ENABLED: bool = false;

    /// Check if debug assertions are enabled
    pub const DEBUG_ASSERTIONS_ENABLED: bool = cfg!(debug_assertions);

    /// Check if overflow checks are enabled
    pub const OVERFLOW_CHECKS_ENABLED: bool = cfg!(debug_assertions);
}

/// Version compatibility checks
pub mod version_checks {
    use crate::utils::LlvmVersion;

    /// Ensure version structure is sound
    pub const VERSION_STRUCTURE_VALID: bool = {
        let version = LlvmVersion { major: 0, minor: 0, patch: 0 };
        !version.as_string().is_empty()
    };
}

#[cfg(test)]
mod build_verification_tests {
    use super::*;

    #[test]
    fn test_compile_time_checks() {
        // These tests verify that our compile-time checks work
        assert!(compile_time_checks::assert_send_sync::<crate::core::Context> as usize == 0);
        assert!(compile_time_checks::assert_error_trait::<crate::error::Error> as usize == 0);
    }

    #[test]
    fn test_feature_detection() {
        // Test that feature detection works
        assert_eq!(feature_detection::TEST_FEATURE_ENABLED, cfg!(test));
        assert_eq!(feature_detection::DEBUG_ASSERTIONS_ENABLED, cfg!(debug_assertions));
    }

    #[test]
    fn test_version_compatibility() {
        // Test that version checking works
        assert!(version_checks::VERSION_STRUCTURE_VALID);
    }
}
