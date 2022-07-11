use std::fmt;
use std::process::Command;
use crate::{TestCase, TestSuite};
use crate::test_execution_environment::TestExecutionEnvironment;

enum TestResult {
    Success,
    Failure(String),
}

pub struct TestExecutionStats {
    pub total_tests: usize,
    pub total_success: usize,
    pub total_failures: usize,
}

impl fmt::Display for TestExecutionStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut stats_message = format!("{} tests run.", self.total_tests).to_string();
        if self.total_success > 0 {
            stats_message.push_str(&format!(" {} passed.", self.total_success));
        }
        if self.total_failures > 0 {
            stats_message.push_str(&format!(" {} failed.", self.total_failures));
        }
        write!(f, "{}", stats_message)
    }
}

impl TestExecutionStats {
    pub fn new() -> TestExecutionStats {
        TestExecutionStats {
            total_tests: 0,
            total_success: 0,
            total_failures: 0,
        }
    }
}

pub struct TestExecutor {
    test_suite: TestSuite,
    execution_environment: TestExecutionEnvironment,
    test_stats: TestExecutionStats,
}

impl TestExecutor {
    pub fn new(test_suite: TestSuite, execution_environment: TestExecutionEnvironment) -> TestExecutor {
        TestExecutor {
            test_suite,
            execution_environment,
            test_stats: TestExecutionStats::new(),
        }
    }

    pub fn execute_all_tests(&mut self) -> &TestExecutionStats {
        if self.test_suite.name.is_some() {
            println!("Tests results for suite '{}':", self.test_suite.name.as_ref().unwrap());
        } else {
            println!("Tests results:");
        }
        self.test_stats = TestExecutionStats::new();
        for test_case in &self.test_suite.tests {
            let result = self.execute_test_case(test_case);
            self.test_stats.total_tests += 1;
            match result {
                TestResult::Success => {
                    self.test_stats.total_success += 1;
                },
                TestResult::Failure(_) => {
                    self.test_stats.total_failures += 1;
                    if self.execution_environment.stop_on_failure {
                        break;
                    }
                }
            }
        }
        &self.test_stats
    }

    fn execute_test_case(&self, test_case: &TestCase) -> TestResult {
        println!("----- {} -----", test_case.name);
        if self.execution_environment.verbose {
            println!("Executing '{}'", test_case.command);
        }
        let mut command = Command::new("/bin/sh");
        command.arg("-c").arg(test_case.command.clone()).current_dir(&self.execution_environment.directory);
        let output = command.output().unwrap();
        let expectations = &test_case.expected;
        let mut success = true;
        if expectations.return_code.is_some() && expectations.return_code.unwrap() != output.status.code().unwrap() {
            println!("Failed");
            println!("Wrong return code: Expected {} but got {}",
                     expectations.return_code.unwrap(), output.status.code().unwrap());
            if self.execution_environment.verbose {
                println!("stdout: '{}'", String::from_utf8_lossy(&output.stdout));
                println!("stderr: '{}'", String::from_utf8_lossy(&output.stderr));
            }
            success = false;
        }
        if expectations.stdout.is_some() && expectations.stdout.as_deref().unwrap() != String::from_utf8_lossy(&output.stdout).to_string() {
            println!("Failed");
            println!("Wrong stdout: Expected '{}' but got '{}'",
                     expectations.stdout.as_deref().unwrap(), String::from_utf8_lossy(&output.stdout).to_string());
            if self.execution_environment.verbose {
                println!("stderr: '{}'", String::from_utf8_lossy(&output.stderr));
            }
            success = false;
        }
        if expectations.stderr.is_some() && expectations.stderr.as_deref().unwrap() != String::from_utf8_lossy(&output.stderr).to_string() {
            println!("Failed");
            println!("Wrong stderr: Expected '{}' but got '{}'",
                     expectations.stderr.as_deref().unwrap(), String::from_utf8_lossy(&output.stderr).to_string());
            if self.execution_environment.verbose {
                println!("stdout: '{}'", String::from_utf8_lossy(&output.stdout));
            }
            success = false;
        }
        if success {
            println!("Passed");
        }
        if !success {
            return TestResult::Failure(test_case.name.clone());
        }
        TestResult::Success
    }
}
