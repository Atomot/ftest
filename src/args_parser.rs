use clap::Parser;
use std::fs::canonicalize;

/// A simple and efficient functional testing tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct ArgsParser {
    /// Directory in which to run the tests
    #[clap(default_value = ".")]
    pub directory: String,

    /// Name of the tests definition file to use
    #[clap(short, long, default_value = ".ftest.yml")]
    pub file: String,

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
        args_parser.relate_file_path_if_not_absolute();
        args_parser
    }

    /// Canonicalizes the directory path so it can be used regardless of the current working directory
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

    /// Makes the file path relative to the directory if it is not absolute
    fn relate_file_path_if_not_absolute(&mut self) {
        match self.file.starts_with('/') {
            true => {}
            false => {
                self.file = format!("{}/{}", self.directory, self.file);
            }
        }
    }
}
