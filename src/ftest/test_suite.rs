use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct CommandExpectation {
    pub return_code: Option<i32>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

impl CommandExpectation {
    pub fn default() -> CommandExpectation {
        CommandExpectation {
            return_code: None,
            stdout: None,
            stderr: None,
        }
    }

    pub fn fill_missing_with(&mut self, other: &CommandExpectation) -> &mut Self {
        if self.return_code.is_none() {
            self.return_code = other.return_code;
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

// TODO Maybe convert the different test types to an enum?
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub r#type: String,
    pub command: String,
    pub expected: Option<CommandExpectation>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandDefaults {
    pub expected: Option<CommandExpectation>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Defaults {
    pub command: Option<CommandDefaults>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestsDefinition {
    pub name: Option<String>,
    pub defaults: Option<Defaults>,
    pub test: Vec<TestCase>,
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
        if test_file.test.is_empty() {
            println!("Warning: Tests definition file does not contain any test");
        }
        test_file
    }
}
