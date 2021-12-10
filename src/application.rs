use crate::state::ApplicationState;
use defmt::*;

// Configure application specific state, pins etc.
// Run after the initial basic setup
pub fn setup(state: &ApplicationState) {
    // let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());
    // let mut led_pin = pins.led.into_push_pull_output();
}

// Application main loop
pub fn application_loop(state: &ApplicationState) {
    let mut delay =
        cortex_m::delay::Delay::new(state.core.SYST, clocks.system_clock.freq().integer());
    let mut led_pin = pins.led.into_push_pull_output();

    info!("on!");
    led_pin.set_high().unwrap();
    delay.delay_ms(500);
    info!("off!");
    led_pin.set_low().unwrap();
    delay.delay_ms(500);
}
