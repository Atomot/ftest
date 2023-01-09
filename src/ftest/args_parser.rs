use clap::Parser;
use std::fs::canonicalize;

const DEFAULT_FILE_NAME: &str = ".ftest.toml";

/// A simple and efficient functional testing tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct ArgsParser {
    /// Directory in which to run the tests
    #[clap(default_value = ".")]
    pub directory: String,

    /// Path of the tests definition file to use
    /// (relative to the current directory; if not specified, will look for a ".ftest.toml" file
    /// in the directory in which the test are run)
    #[clap(short, long)]
    file: Option<String>,

    /// Stop after the first failure
    #[clap(short, long, value_parser, default_value_t = false)]
    pub stop_on_failure: bool,

    /// Output more information about the tests
    #[clap(short, long, value_parser, default_value_t = false)]
    pub verbose: bool,
}

impl ArgsParser {
    pub fn parse_args() -> Self {
        let mut args_parser: ArgsParser = ArgsParser::parse();
        args_parser.canonicalize_directory();
        //args_parser.relate_file_path_if_not_absolute();
        args_parser
    }

    // Canonicalizes the directory path so it can be used regardless of the current working directory
    fn canonicalize_directory(&mut self) {
        let directory_full_path_result = canonicalize(self.directory.trim_end_matches('/'));
        let directory_full_path = match directory_full_path_result {
            Ok(path) => path,
            Err(e) => {
                println!("Error while parsing directory path: {}", e);
                std::process::exit(3);
            }
        };
        let directory = match directory_full_path.to_str() {
            Some(str) => str,
            None => {
                println!("Error while parsing directory path: Unsupported character set");
                std::process::exit(3);
            }
        };
        self.directory = String::from(directory);
    }

    // Get the tests definition file path
    pub fn get_file_path(&self) -> String {
        match &self.file {
            None => format!("{}/{}", self.directory, DEFAULT_FILE_NAME),
            Some(file) => file.clone(),
        }
    }
}
