// use defmt::*;
use defmt_rtt as _;

// use embedded_hal::digital::v2::OutputPin;
use embedded_hal::PwmPin;
// use pico::hal::gpio::{bank0::Gpio25, Pin, PushPullOutput};
use pico::hal::pwm::{Channel, FreeRunning, Pwm4, B};

// The minimum PWM value (i.e. LED brightness) we want
const LOW: u16 = 0;

// The maximum PWM value (i.e. LED brightness) we want
const HIGH: u16 = 25000;

pub fn application_main(
    //mut led_pin: Pin<Gpio25, PushPullOutput>,
    pwm_channel: &mut Channel<Pwm4, FreeRunning, B>,
    mut delay: cortex_m::delay::Delay,
) -> ! {
    loop {
        // Ramp brightness up
        for i in (LOW..=HIGH).skip(100) {
            delay.delay_us(8);
            pwm_channel.set_duty(i);
        }

        // Ramp brightness down
        for i in (LOW..=HIGH).rev().skip(100) {
            delay.delay_us(8);
            pwm_channel.set_duty(i);
        }

        delay.delay_ms(500);
    }
}
