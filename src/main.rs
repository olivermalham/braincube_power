#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

mod application;
mod state;

use crate::state::ApplicationState;

#[entry]
fn main() -> ! {
    info!("Program start");

    let state = ApplicationState::new();

    // Mimic Ardunio code structure to try and keep application code clear of boiler plate
    application::setup(&state);

    loop {
        application::application_loop(&state);
    }
}

// End of file
