mod app;
mod crossterm;

use std::{error::Error, time::Duration};

use crate::crossterm::run;

const TICK_RATE: Duration = Duration::from_millis(100);

fn main() -> Result<(), Box<dyn Error>> {
    run(TICK_RATE)?;
    Ok(())
}
