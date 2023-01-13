use crate::test_execution_environment::TestExecutionEnvironment;
use crate::test_executor::TestResult;
use serde::Deserialize;
use std::process::Command;

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct CommandExpectation {
    pub exit_code: Option<i32>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

impl CommandExpectation {
    pub fn default() -> CommandExpectation {
        CommandExpectation {
            exit_code: None,
            stdout: None,
            stderr: None,
        }
    }

    pub fn fill_missing_with(&mut self, other: &CommandExpectation) -> &mut Self {
        if self.exit_code.is_none() {
            self.exit_code = other.exit_code;
        }
        if self.stdout.is_none() {
            self.stdout = other.stdout.clone();
        }
        if self.stderr.is_none() {
            self.stderr = other.stderr.clone();
        }
        self
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct CommandTestCase {
    pub command: String,
    #[serde(flatten)]
    pub params: CommandParams,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommandParams {
    pub expected: Option<CommandExpectation>,
}

impl CommandTestCase {
    fn execute_test_case(
        &self,
        execution_environment: &TestExecutionEnvironment,
        defaults: Option<&CommandParams>,
    ) -> TestResult {
        if execution_environment.verbose {
            println!("Executing '{}'", self.command);
        }
        let mut command = Command::new("/bin/sh");
        command
            .arg("-c")
            .arg(self.command.clone())
            .current_dir(&execution_environment.directory);
        let output = command.output().unwrap();
        let mut expectations = self
            .params
            .expected
            .clone()
            .unwrap_or_else(CommandExpectation::default);
        if let Some(command_defaults) = defaults.and_then(|d| d.expected.as_ref()) {
            expectations.fill_missing_with(command_defaults);
        }
        let mut success = true;
        if expectations.exit_code.is_some()
            && expectations.exit_code.unwrap() != output.status.code().unwrap()
        {
            println!(
                "Wrong exit code: Expected {} but got {}",
                expectations.exit_code.unwrap(),
                output.status.code().unwrap()
            );
            if execution_environment.verbose {
                println!("stdout: '{}'", String::from_utf8_lossy(&output.stdout));
                println!("stderr: '{}'", String::from_utf8_lossy(&output.stderr));
            }
            success = false;
        }
        if expectations.stdout.is_some()
            && expectations.stdout.as_deref().unwrap() != String::from_utf8_lossy(&output.stdout)
        {
            println!(
                "Wrong stdout: Expected '{}' but got '{}'",
                expectations.stdout.as_deref().unwrap(),
                String::from_utf8_lossy(&output.stdout)
            );
            if execution_environment.verbose {
                println!("stderr: '{}'", String::from_utf8_lossy(&output.stderr));
            }
            success = false;
        }
        if expectations.stderr.is_some()
            && expectations.stderr.as_deref().unwrap() != String::from_utf8_lossy(&output.stderr)
        {
            println!(
                "Wrong stderr: Expected '{}' but got '{}'",
                expectations.stderr.as_deref().unwrap(),
                String::from_utf8_lossy(&output.stderr)
            );
            if execution_environment.verbose {
                println!("stdout: '{}'", String::from_utf8_lossy(&output.stdout));
            }
            success = false;
        }
        if success {
            TestResult::Success
        } else {
            TestResult::Failure
        }
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum PathTargetType {
    File,
    Directory,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct PathExistsTestCase {
    pub path: String,
    #[serde(flatten)]
    pub params: PathExistsParams,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PathExistsParams {
    pub target_type: Option<PathTargetType>,
}

impl PathExistsTestCase {
    fn execute_test_case(
        &self,
        execution_environment: &TestExecutionEnvironment,
        defaults: Option<&PathExistsParams>,
    ) -> TestResult {
        let path = std::path::Path::new(&execution_environment.directory).join(&self.path);
        if execution_environment.verbose {
            println!("Checking existence of '{}'", path.display());
        }
        if !path.exists() {
            println!("'{}' does not exist", self.path);
            return TestResult::Failure;
        }
        let target_type = self.params.target_type.clone().unwrap_or_else(|| {
            defaults
                .and_then(|d| d.target_type.as_ref())
                .cloned()
                .unwrap_or_else(|| {
                    if path.is_dir() {
                        PathTargetType::Directory
                    } else {
                        PathTargetType::File
                    }
                })
        });
        match target_type {
            PathTargetType::File => {
                if !path.is_file() {
                    println!("'{}' is not a file", self.path);
                    return TestResult::Failure;
                }
            }
            PathTargetType::Directory => {
                if !path.is_dir() {
                    println!("'{}' is not a directory", self.path);
                    return TestResult::Failure;
                }
            }
        }
        TestResult::Success
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, tag = "type", rename_all = "snake_case")]
pub enum TestCaseType {
    Dummy,
    Command(CommandTestCase),
    PathExists(PathExistsTestCase),
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct TestCase {
    pub name: String,
    #[serde(flatten)]
    pub test_data: TestCaseType,
}

impl TestCase {
    pub fn execute_test_case(
        &self,
        execution_environment: &TestExecutionEnvironment,
        defaults: Option<&Defaults>,
    ) -> TestResult {
        match &self.test_data {
            TestCaseType::Dummy => TestResult::Success,
            TestCaseType::Command(command_test_case) => command_test_case.execute_test_case(
                execution_environment,
                defaults.and_then(|d| d.command.as_ref()),
            ),
            TestCaseType::PathExists(path_exists_test_case) => path_exists_test_case
                .execute_test_case(
                    execution_environment,
                    defaults.and_then(|d| d.path_exists.as_ref()),
                ),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Defaults {
    pub command: Option<CommandParams>,
    pub path_exists: Option<PathExistsParams>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestsDefinition {
    pub name: Option<String>,
    pub always_stop_after_failure: Option<bool>,
    pub defaults: Option<Defaults>,
    pub test: Option<Vec<TestCase>>,
}

impl TestsDefinition {
    pub fn load_from_file(path: &str) -> TestsDefinition {
        // Reads the tests file and parses it into a TestsDefinitionFile struct.
        let file_str = match std::fs::read_to_string(path) {
            Ok(str) => str,
            Err(e) => {
                println!(
                    "Error while trying to read the tests definition file {}: {}",
                    path, e
                );
                std::process::exit(3);
            }
        };
        if file_str.trim().is_empty() {
            println!("The tests definition file {} is empty", path);
            std::process::exit(3);
        }
        let test_file: TestsDefinition = match toml::from_str(&file_str) {
            Ok(test_file) => test_file,
            Err(e) => {
                println!(
                    "Error while trying to parse the tests definition file {}: {}",
                    path, e
                );
                std::process::exit(3);
            }
        };
        if test_file.test.is_none() || test_file.test.as_ref().unwrap().is_empty() {
            println!("Warning: Tests definition file does not contain any test");
        }
        test_file
    }
}
