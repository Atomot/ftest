use crate::test_execution_environment::TestExecutionEnvironment;
use crate::TestsDefinition;
use colored::Colorize;

pub enum TestResult {
    Success,
    Failure,
}

pub struct TestExecutionStats {
    pub total_tests: usize,
    pub total_success: usize,
    pub total_failures: usize,
}

impl TestExecutionStats {
    pub(crate) fn print(&self) {
        let plural_indicator = if self.total_tests > 1 { "s" } else { "" };
        print!(
            "{}",
            format!("{} test{} run.", self.total_tests, plural_indicator).normal()
        );
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
        if self.test_suite.test.is_none() || self.test_suite.test.as_ref().unwrap().is_empty() {
            println!("No tests to execute.");
            return &self.test_stats;
        }
        if self.test_suite.name.is_some() {
            println!(
                "Tests results for suite '{}':",
                self.test_suite.name.as_ref().unwrap()
            );
        } else {
            println!("Tests results:");
        }
        self.test_stats = TestExecutionStats::new();
        for test_case in self.test_suite.test.as_ref().unwrap() {
            println!("----- {} -----", test_case.name);
            let result = test_case.execute_test_case(
                &self.execution_environment,
                self.test_suite.defaults.as_ref(),
            );
            match result {
                TestResult::Success => println!("{}", "Passed".green()),
                TestResult::Failure => println!("{}", "Failed".red().bold()),
            }
            self.test_stats.total_tests += 1;
            match result {
                TestResult::Success => {
                    self.test_stats.total_success += 1;
                }
                TestResult::Failure => {
                    self.test_stats.total_failures += 1;
                    if self.execution_environment.stop_on_failure
                        || self.test_suite.always_stop_after_failure.unwrap_or(false)
                    {
                        println!(
                            "{}",
                            "\nStopping execution on first failure.".yellow().italic()
                        );
                        break;
                    }
                }
            }
        }
        &self.test_stats
    }
}
