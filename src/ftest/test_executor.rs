use crate::test_execution_environment::TestExecutionEnvironment;
use crate::test_suite::CommandExpectation;
use crate::{TestCase, TestsDefinition};
use colored::Colorize;
use std::process::Command;

enum TestResult {
    Success,
    Failure(String),
}

pub struct TestExecutionStats {
    pub total_tests: usize,
    pub total_success: usize,
    pub total_failures: usize,
}

impl TestExecutionStats {
    pub(crate) fn print(&self) {
        print!("{}", format!("{} tests run.", self.total_tests).normal());
        if self.total_success > 0 {
            print!("{}", format!(" {} passed.", self.total_success).green());
        }
        if self.total_failures > 0 {
            print!(
                "{}",
                format!(" {} failed.", self.total_failures).red().bold()
            );
        }
        println!();
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
    test_suite: TestsDefinition,
    execution_environment: TestExecutionEnvironment,
    test_stats: TestExecutionStats,
}

impl TestExecutor {
    pub fn new(
        test_suite: TestsDefinition,
        execution_environment: TestExecutionEnvironment,
    ) -> TestExecutor {
        TestExecutor {
            test_suite,
            execution_environment,
            test_stats: TestExecutionStats::new(),
        }
    }

    pub fn execute_all_tests(&mut self) -> &TestExecutionStats {
        if self.test_suite.name.is_some() {
            println!(
                "Tests results for suite '{}':",
                self.test_suite.name.as_ref().unwrap()
            );
        } else {
            println!("Tests results:");
        }
        self.test_stats = TestExecutionStats::new();
        for test_case in &self.test_suite.test {
            let result = self.execute_test_case(test_case);
            self.test_stats.total_tests += 1;
            match result {
                TestResult::Success => {
                    self.test_stats.total_success += 1;
                }
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
        command
            .arg("-c")
            .arg(test_case.command.clone())
            .current_dir(&self.execution_environment.directory);
        let output = command.output().unwrap();
        let mut expectations = test_case
            .expected
            .clone()
            .unwrap_or_else(CommandExpectation::default);
        if let Some(command_defaults) = self
            .test_suite
            .defaults
            .as_ref()
            .and_then(|d| d.command.as_ref().and_then(|c| c.expected.as_ref()))
        {
            expectations.fill_missing_with(command_defaults);
        }
        let mut success = true;
        let failed_message = "Failed".red().bold();
        if expectations.return_code.is_some()
            && expectations.return_code.unwrap() != output.status.code().unwrap()
        {
            println!("{}", failed_message);
            println!(
                "Wrong return code: Expected {} but got {}",
                expectations.return_code.unwrap(),
                output.status.code().unwrap()
            );
            if self.execution_environment.verbose {
                println!("stdout: '{}'", String::from_utf8_lossy(&output.stdout));
                println!("stderr: '{}'", String::from_utf8_lossy(&output.stderr));
            }
            success = false;
        }
        if expectations.stdout.is_some()
            && expectations.stdout.as_deref().unwrap() != String::from_utf8_lossy(&output.stdout)
        {
            if success {
                println!("{}", failed_message);
            }
            println!(
                "Wrong stdout: Expected '{}' but got '{}'",
                expectations.stdout.as_deref().unwrap(),
                String::from_utf8_lossy(&output.stdout)
            );
            if self.execution_environment.verbose {
                println!("stderr: '{}'", String::from_utf8_lossy(&output.stderr));
            }
            success = false;
        }
        if expectations.stderr.is_some()
            && expectations.stderr.as_deref().unwrap() != String::from_utf8_lossy(&output.stderr)
        {
            if success {
                println!("{}", failed_message);
            }
            println!(
                "Wrong stderr: Expected '{}' but got '{}'",
                expectations.stderr.as_deref().unwrap(),
                String::from_utf8_lossy(&output.stderr)
            );
            if self.execution_environment.verbose {
                println!("stdout: '{}'", String::from_utf8_lossy(&output.stdout));
            }
            success = false;
        }
        if success {
            println!("{}", "Passed".green());
        }
        if !success {
            return TestResult::Failure(test_case.name.clone());
        }
        TestResult::Success
    }
}
