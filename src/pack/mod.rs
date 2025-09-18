use std::sync::RwLock;
use anyhow::Result;
use stopwatch::Stopwatch;
use crate::Phase;

pub struct PackPhase {
    stopwatch: RwLock<Stopwatch>
}

impl Phase for PackPhase {
    fn new() -> Self {
        PackPhase {
            stopwatch: RwLock::new(Stopwatch::new())
        }
    }

    fn run(&self) -> Result<i32> {
        let mut duration = self.stopwatch.write().unwrap();
        duration.start();
        drop(duration);

        println!("{} phase is not yet implemented", self.name());

        let mut duration = self.stopwatch.write().unwrap();
        duration.stop();
        Ok(0)
    }

    fn duration(&self) -> std::time::Duration {
        let duration = self.stopwatch.read().unwrap();
        duration.elapsed()
    }
}
