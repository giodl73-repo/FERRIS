mod entrypoint;

use std::process::ExitCode;

fn main() -> ExitCode {
    entrypoint::main_exit_code()
}
