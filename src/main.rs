#![no_main]
#![no_std]

use crate::hal::{pac, prelude::*};
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal as hal;

const DOT_LENGTH: u32 = 10_000;
const PAUSE_LENGTH: u32 = 10_000;
const LONG_PAUSE_LENGTH: u32 = 20_000;
const LINE_LENGTH: u32 = 20_000;

#[entry]
fn main() -> ! {
    let message = "...___... ";
    let p = pac::Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();
    let mut led = gpioa.pa1.into_push_pull_output();

    loop {
        for character in message.chars().into_iter() {
            match character {
                '.' => {
                    for _ in 1..DOT_LENGTH {
                        led.set_high();
                    }
                }
                '_' => {
                    for _ in 1..LINE_LENGTH {
                        led.set_high();
                    }
                }
                ' ' => {
                    for _ in 1..LONG_PAUSE_LENGTH {
                        led.set_low();
                    }
                }
                _ => {}
            }
            for _ in 1..PAUSE_LENGTH {
                led.set_low();
            }
        }
    }
}
