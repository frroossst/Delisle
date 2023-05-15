//! Blinks an LED

#![no_main]
#![no_std]

use stm32f4xx_hal as hal;
use panic_halt as _;

use crate::hal::{pac, prelude::*};
use cortex_m_rt::entry;


#[entry]
fn main() -> ! 
    {
    let p = pac::Peripherals::take().unwrap();
    let gpioc = p.GPIOC.split();

    let mut led = gpioc.pc13.into_push_pull_output();

    loop 
        {
        for _ in 0..10_000 
            {
            led.set_high();
            }
        for _ in 0..10_000 
            {
            led.set_low();
            }
        }
    }
