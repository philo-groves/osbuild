use crate::prepare::args::BuildArgs;
use anyhow::Result;

pub mod prepare;
pub mod build;
pub mod pack;
pub mod boot;

pub fn process_command() -> Result<i32> {
    Ok(0)
}

pub fn process_runner(args: BuildArgs) -> Result<i32> {
    Ok(0)
}
