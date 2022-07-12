use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandExpectation {
    pub return_code: Option<i32>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

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
    pub tests: Vec<TestCase>,
}

impl TestSuite {
    pub fn load_from_file(path: &str) -> TestSuite {
        // Reads the tests file and parses it into a TestFile struct.
        let file_read_result = std::fs::read_to_string(path);
        if file_read_result.is_err() {
            println!(
                "Error while trying to read the tests definition file: {}",
                file_read_result.unwrap_err()
            );
            std::process::exit(3);
        }
        let yaml_str = file_read_result.unwrap();
        let test_file_load_result: Result<TestSuite, serde_yaml::Error> =
            serde_yaml::from_str(&yaml_str);
        if test_file_load_result.is_err() {
            println!(
                "Error while trying to parse the tests definition file: {}",
                test_file_load_result.unwrap_err()
            );
            std::process::exit(3);
        }
        let test_file = test_file_load_result.unwrap();
        if test_file.tests.is_empty() {
            println!("Warning: Tests definition file does not contain any test");
        }
        test_file
    }
}
