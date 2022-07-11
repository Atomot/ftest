mod args_parser;
mod test_suite;
mod test_executor;
mod test_execution_environment;

use crate::args_parser::ArgsParser;
use crate::test_execution_environment::TestExecutionEnvironment;
use crate::test_executor::TestExecutor;
use crate::test_suite::{TestCase, TestSuite};

fn main() {
    let args: ArgsParser = ArgsParser::parse_args();
    let test_suite: TestSuite = TestSuite::load_from_file(&args.file);
    let execution_environment = TestExecutionEnvironment {
        directory: args.directory,
        stop_on_failure: args.stop_on_failure,
        verbose: args.verbose,
    };
    let mut test_executor = TestExecutor::new(test_suite, execution_environment);
    let tests_stats = test_executor.execute_all_tests();
    println!();
    println!("{}", tests_stats);
    if tests_stats.total_failures == 0 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}
