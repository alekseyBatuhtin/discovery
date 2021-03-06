//! Initialization code

#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate f3;
extern crate panic_abort;

use core::sync::atomic::{self, Ordering};

use cortex_m::asm;
use cortex_m_rt::ExceptionFrame;
pub use f3::hal::delay::Delay;
pub use f3::hal::prelude;
use f3::hal::prelude::*;
use f3::hal::stm32f30x;
pub use f3::led::Leds;

pub fn init() -> (Delay, Leds) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let delay = Delay::new(cp.SYST, clocks);

    let leds = Leds::new(dp.GPIOE.split(&mut rcc.ahb));

    (delay, leds)
}

exception!(HardFault, hard_fault);

fn hard_fault(_ef: &ExceptionFrame) -> ! {
    asm::bkpt();

    loop {
        // add some side effect to prevent LLVM from turning this into a UDF (abort) instruction
        // see rust-lang/rust#28728 for details
        atomic::compiler_fence(Ordering::SeqCst)
    }
}

exception!(*, default_handler);

fn default_handler(_irqn: i16) {
    loop {
        atomic::compiler_fence(Ordering::SeqCst)
    }
}
