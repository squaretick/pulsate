//! The `p8` binary — a short alias for `pulsate`.
#![forbid(unsafe_code)]

fn main() -> std::process::ExitCode {
    pulsate::run()
}
