// Main entry point for command line interface (CLI).
fn main() {
    osbuild::process_command()
        .map_err(|e| eprintln!("Error: {}", e))
        .ok();
}

/// This runner is executed when `cargo build` is used with Rust-based projects.
pub(crate) fn runner(args: osbuild::prepare::args::BuildArgs) -> anyhow::Result<i32> {
    osbuild::process_runner(args)
}
