//! Test utilities and validation macros

use crate::error::{SzError, SzResult};

/// Tracks test failures and ensures no silent failures
pub struct TestValidator {
    failures: Vec<String>,
    test_name: String,
}

impl TestValidator {
    pub fn new(test_name: &str) -> Self {
        Self {
            failures: Vec::new(),
            test_name: test_name.to_string(),
        }
    }

    /// Record a failure but continue test execution for diagnostic purposes
    pub fn record_failure(&mut self, failure: String) {
        println!("❌ FAILURE: {}", failure);
        self.failures.push(failure);
    }

    /// Record a success
    pub fn record_success(&self, message: String) {
        println!("✅ SUCCESS: {}", message);
    }

    /// Check if operation succeeded, record failure if not
    pub fn check_result<T>(&mut self, result: SzResult<T>, operation: &str) -> Option<T> {
        match result {
            Ok(value) => {
                self.record_success(format!("{} succeeded", operation));
                Some(value)
            }
            Err(e) => {
                self.record_failure(format!("{} failed: {}", operation, e));
                None
            }
        }
    }

    /// Validate a condition, record failure if false
    pub fn validate(&mut self, condition: bool, message: &str) {
        if condition {
            self.record_success(message.to_string());
        } else {
            self.record_failure(format!("Validation failed: {}", message));
        }
    }

    /// Finalize test - return error if any failures occurred
    pub fn finalize(self) -> SzResult<()> {
        if self.failures.is_empty() {
            println!("🎯 Test '{}' completed successfully", self.test_name);
            Ok(())
        } else {
            let error_msg = format!(
                "Test '{}' failed with {} errors:\n{}",
                self.test_name,
                self.failures.len(),
                self.failures.join("\n")
            );
            println!("💥 {}", error_msg);
            Err(SzError::configuration(error_msg))
        }
    }
}

/// Macro for mandatory verification
#[macro_export]
macro_rules! verify_or_fail {
    ($validator:expr, $condition:expr, $message:expr) => {
        if !($condition) {
            $validator.record_failure(format!("Verification failed: {}", $message));
        } else {
            $validator.record_success(format!("Verified: {}", $message));
        }
    };
}

/// Macro for operations that must succeed
#[macro_export]
macro_rules! must_succeed {
    ($validator:expr, $result:expr, $operation:expr) => {
        match $result {
            Ok(value) => {
                $validator.record_success(format!("{} succeeded", $operation));
                value
            }
            Err(e) => {
                $validator.record_failure(format!("{} failed: {}", $operation, e));
                return $validator.finalize();
            }
        }
    };
}

/// Macro for data source registration verification
#[macro_export]
macro_rules! verify_data_source_registration {
    ($validator:expr, $config_mgr:expr, $engine:expr, $data_source:expr, $config_id:expr) => {{
        // Verify in configuration
        let config = $validator.check_result(
            $config_mgr.create_config_from_id($config_id),
            &format!("Creating config from ID {}", $config_id)
        );

        if let Some(config) = config {
            let registry = $validator.check_result(
                config.get_data_source_registry(),
                &format!("Getting data source registry for {}", $data_source)
            );

            if let Some(registry) = registry {
                $validator.validate(
                    registry.contains($data_source),
                    &format!("Data source {} found in registry", $data_source)
                );
            }
        }

        // Verify engine functionality
        let test_record = format!(r#"{{"DATA_SOURCE": "{}", "RECORD_ID": "TEST_{}", "NAME_FULL": "Test Person"}}"#, $data_source, $data_source);
        let add_result = $engine.add_record($data_source, &format!("TEST_{}", $data_source), &test_record, None);

        match add_result {
            Ok(_) => {
                $validator.record_success(format!("Data source {} is functionally working", $data_source));
                // Clean up
                let _ = $engine.delete_record($data_source, &format!("TEST_{}", $data_source), None);
            }
            Err(e) => {
                $validator.record_failure(format!("Data source {} is not functional: {}", $data_source, e));
            }
        }
    }};
}