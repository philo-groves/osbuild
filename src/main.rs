use anyhow::Result;
use osbuild::prepare::args::BuildArgs;

// Main entry point for command line interface (CLI).
fn main() {
    osbuild::process_command()
        .map_err(|e| eprintln!("Error: {}", e))
        .ok();
}

/// This runner is executed when `cargo build` is used with Rust-based projects.
pub(crate) fn runner(args: BuildArgs) -> Result<i32> {
    osbuild::process_runner(args)
}
