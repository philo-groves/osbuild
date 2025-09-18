use std::sync::RwLock;
use anyhow::Result;
use stopwatch::Stopwatch;
use crate::Phase;

pub struct RunPhase {
    stopwatch: RwLock<Stopwatch>
}

impl Phase for RunPhase {
    fn new() -> Self {
        RunPhase {
            stopwatch: RwLock::new(Stopwatch::new())
        }
    }

    fn run(&self) -> Result<i32> {
        let mut duration = self.stopwatch.write().unwrap();
        duration.start();
        drop(duration);

        println!("The run phase is not yet implemented");

        let mut duration = self.stopwatch.write().unwrap();
        duration.stop();
        Ok(0)
    }

    fn duration(&self) -> std::time::Duration {
        let duration = self.stopwatch.read().unwrap();
        duration.elapsed()
    }
}
