//! The `pulsate` binary — a thin wrapper around [`pulsate::run`].
#![forbid(unsafe_code)]

fn main() -> std::process::ExitCode {
    pulsate::run()
}
