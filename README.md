# `ftest` - A simple and efficient functional testing tool

`ftest` is a command-line tool used for functional testing.

It is easily configurable via an easy-to-read (and easy-to-write) TOML configuration file.

## Installation

### RPM package (Fedora, CentOS, RHEL)

You can easily install `ftest` via the RPM package, by downloading the latest release from the releases page.

You can then install the package with the following command:

```bash
sudo dnf install ftest-<version>.rpm
```

(This also works with `yum`.)

## Usage

It will look for a `.ftest.toml` file in the current directory, and use that to run the tests.

### Configuration

The configuration file is a TOML file. In its simplest form, it looks like this:

```toml
name = "simple_add test suite"

[[test]]
name = "Basic sum"
type = "command"
command = "./simple_add 1 2"
expected.stdout = "3\n"
expected.stderr = ""
expected.exit_code = 0

[[test]]
name = "No arguments"
type = "command"
command = "./simple_add"
# Any error message is fine, but it must be displayed on stderr
expected.stdout = ""
expected.exit_code = 1
```

More information about the configuration file can be found in the [configuration documentation](docs/configuration.md).

### Running tests

To run the tests, simply run `ftest` in the directory containing the configuration file.

```
$> ftest
Tests results for suite 'simple_add tests':
----- Basic sum -----
Passed
----- No arguments -----
Passed

2 tests run. 2 passed.
```

You can pass several flags and arguments to customize the behaviour of `ftest`. You can see the list of available flags and arguments by running `ftest -h`.

