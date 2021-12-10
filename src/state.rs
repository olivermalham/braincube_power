use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
// use pico as bsp;
// use pro_micro_rp2040 as bsp;

use pico::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

pub struct ApplicationState {
    //pub pac: pac::Peripherals,
    pub core: pac::CorePeripherals,
    pub watchdog: Watchdog,
    //pub sio: Sio,
    pub pins: pico::Pins,
    pub clocks: pico::hal::clocks::ClocksManager,
}

impl ApplicationState {
    pub fn new() -> ApplicationState {
        let mut pac = pac::Peripherals::take().unwrap();
        let core = pac::CorePeripherals::take().unwrap();
        let mut watchdog = Watchdog::new(pac.WATCHDOG);
        let sio = Sio::new(pac.SIO);

        // External high-speed crystal on the pico board is 12Mhz
        let external_xtal_freq_hz = 12_000_000u32;
        let clocks = init_clocks_and_plls(
            external_xtal_freq_hz,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        let pins = pico::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        ApplicationState {
            //pac: pac,
            core: core,
            watchdog: watchdog,
            //sio: sio,
            pins: pins,
        }
    }
}
