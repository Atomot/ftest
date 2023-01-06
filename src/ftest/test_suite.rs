use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandExpectation {
    pub return_code: Option<i32>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

// TODO Maybe convert the different test types to an enum?
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub r#type: String,
    pub command: String,
    pub expected: CommandExpectation,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TestSuite {
    pub name: Option<String>,
    pub test: Vec<TestCase>,
}

impl TestSuite {
    pub fn load_from_file(path: &str) -> TestSuite {
        // Reads the tests file and parses it into a TestFile struct.
        let file_str = match std::fs::read_to_string(path) {
            Ok(str) => str,
            Err(e) => {
                println!(
                    "Error while trying to read the tests definition file: {}",
                    e
                );
                std::process::exit(3);
            }
        };
        let test_file: TestSuite = match toml::from_str(&file_str) {
            Ok(test_file) => test_file,
            Err(e) => {
                println!(
                    "Error while trying to parse the tests definition file: {}",
                    e
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
