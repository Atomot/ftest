# Configuration

The configuration file for `ftest` is a TOML file. It is used to define the tests to run, and the expected results, and eventual additional behaviours.

## Root properties

The properties that can be defined at the root of the configuration file are:

- `name` (optional): The name of the test suite, as a string. This is used to display the name of the test suite when running the tests.
- `always_stop_after_failure` (optional): A boolean value. If set to `true`, the tests will stop running after the first failure. If set to `false`, the tests will continue running even if a test fails (unless the `-s` flag is used when running `ftest`). Defaults to `false`.

## Defaults

The `defaults` property can be used to define default values for the tests.

## Tests definitions

The tests are defined in the `test` array. Each test is defined by the following properties:

- `name` **(required)**: The name of the test, as a string.
- `type` **(required)**: The type of the test. Can be one of the values of the next section.

### `command`

Executes a shell command and checks the results.

It must include a `command` property, which contains a string representing the command to execute.

An `expected` object can then be specified, containing the expected results of the command. It can contain the following properties:

- `stdout`: The expected standard output of the command, as a string.
- `stderr`: The expected standard error of the command, as a string.
- `exit_code`: The expected exit code of the command, as an integer.

If one of these properties is not specified, it will not be checked.

Example:

```toml
[[test]]
name = "Basic sum"
type = "command"
command = "./simple_add 1 2"
expected.stdout = "3\n"
expected.stderr = ""
expected.exit_code = 0
```

### `path_exists`

Checks that a path exists, and optionally checks its type.

It must include a `path` property, which contains a string representing the path to check.
The test will fail if the path does not exist.

Additionally, a `target_type` property can be specified, which can be one either `file` or `directory`. If specified, the test will fail if the target is not of the specified type.

Example:

```toml
[[test]]
name = "src/Makefile exists"
type = "path_exists"
path = "src/Makefile"
target_type = "file"
```
