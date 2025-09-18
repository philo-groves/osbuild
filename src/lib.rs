pub mod prepare;
pub mod compile;
pub mod pack;
pub mod run;

use std::time::Duration;
use anyhow::Result;
use crate::{compile::CompilePhase, pack::PackPhase, prepare::{args::RunnerArgs, PreparePhase}, run::RunPhase};

pub fn process_command() -> Result<i32> {
    process()
}

pub fn process_runner(args: RunnerArgs) -> Result<i32> {
    process()
}

fn process() -> Result<i32> {
    for phase in phases() {
        println!("Starting {} phase", phase.short_name());
        phase.run()?;
        println!("Completed {} phase in {:?}ms", phase.short_name(), phase.duration().as_millis());
    }
    
    Ok(0)
}

fn phases() -> Vec<Box<dyn Phase>> {
    vec![
        Box::new(PreparePhase::new()),
        Box::new(CompilePhase::new()),
        Box::new(PackPhase::new()),
        Box::new(RunPhase::new())
    ]
}

pub trait Phase {
    fn new() -> Self where Self: Sized;
    fn run(&self) -> Result<i32>;
    fn duration(&self) -> Duration;

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn short_name(&self) -> String {
        self.name()
            .split("::")
            .last()
            .unwrap_or_default()
            .trim_end_matches("Phase")
            .to_uppercase()
    }
}