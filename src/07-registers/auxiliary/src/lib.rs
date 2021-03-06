//! Initialization code

#![feature(panic_implementation)]
#![no_std]

#[macro_use]
extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;
extern crate f3;

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

use cortex_m::asm;
pub use cortex_m::asm::bkpt;
pub use cortex_m::peripheral::ITM;
use cortex_m_rt::ExceptionFrame;
use f3::hal::prelude::*;
pub use f3::hal::stm32f30x::gpioc;
use f3::hal::stm32f30x::{self, GPIOE};
use f3::led::Leds;

#[inline(never)]
pub fn init() -> (ITM, &'static gpioc::RegisterBlock) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    Leds::new(dp.GPIOE.split(&mut rcc.ahb));

    (cp.ITM, unsafe { &*GPIOE::ptr() })
}

#[allow(deprecated)]
#[panic_implementation]
fn panic(info: &PanicInfo) -> ! {
    let itm = unsafe { &mut *ITM::ptr() };

    iprintln!(&mut itm.stim[0], "{}", info);

    cortex_m::asm::bkpt();

    loop {
        // add some side effect to prevent LLVM from turning this loop into a UDF (abort) instruction
        // see rust-lang/rust#28728 for details
        atomic::compiler_fence(Ordering::SeqCst)
    }
}

exception!(HardFault, hard_fault);

fn hard_fault(_ef: &ExceptionFrame) -> ! {
    asm::bkpt();

    loop {
        atomic::compiler_fence(Ordering::SeqCst)
    }
}

exception!(*, default_handler);

fn default_handler(_irqn: i16) {
    loop {
        atomic::compiler_fence(Ordering::SeqCst)
    }
}
