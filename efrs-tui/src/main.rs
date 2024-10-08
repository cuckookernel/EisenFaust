mod app;
mod app_util;
mod exec_cell;
#[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(feature = "termion")]
mod termion;
mod ui;
mod util;

#[cfg(feature = "crossterm")]
use crate::crossterm::run;

#[cfg(feature = "termion")]
use crate::termion::run;
use argh::FromArgs;
use std::{error::Error, time::Duration};

/// Demo
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    enhanced_graphics: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Attempting to start EisenFaust-rs TUI");
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    run(tick_rate, cli.enhanced_graphics)?;

    println!("EisenFaust-rs TUI ended");
    Ok(())
}