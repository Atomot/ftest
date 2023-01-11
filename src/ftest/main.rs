mod args_parser;
mod test_execution_environment;
mod test_executor;
mod test_suite;

use crate::args_parser::ArgsParser;
use crate::test_execution_environment::TestExecutionEnvironment;
use crate::test_executor::TestExecutor;
use crate::test_suite::TestsDefinition;

fn main() {
    let args: ArgsParser = ArgsParser::parse_args();
    let tests_definition: TestsDefinition = TestsDefinition::load_from_file(&args.get_file_path());
    let execution_environment = TestExecutionEnvironment {
        directory: args.directory,
        stop_on_failure: args.stop_on_failure,
        verbose: args.verbose,
    };
    let mut test_executor = TestExecutor::new(tests_definition, execution_environment);
    let tests_stats = test_executor.execute_all_tests();
    println!();
    tests_stats.print();
    if tests_stats.total_failures == 0 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}
