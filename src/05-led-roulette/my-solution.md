# My solution

What solution did you come up with?

Here's mine:

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate aux5;
#[macro_use]
extern crate cortex_m_rt;

use aux5::prelude::*;
use aux5::{Delay, Leds};

entry!(main);

fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let ms = 50_u8;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on();
            delay.delay_ms(ms);
            leds[curr].off();
            delay.delay_ms(ms);
        }
    }
}
```

One more thing! Check that your solution also works when compiled in "release" mode:

``` console
$ cargo build --target thumbv7em-none-eabihf --release
```

You can test it with this `gdb` command:

``` console
$ # or, you could simply call `cargo run --target thumbv7em-none-eabihf --release`
$ arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/led-roulette
$ #                                              ~~~~~~~
```

Binary size is something we should always keep an eye on! How big is your solution? You can check
that using the `size` command on the release binary:

``` console
$ # equivalent to size target/thumbv7em-none-eabihf/debug/led-roulette
$ cargo size --target thumbv7em-none-eabihf --bin led-roulette -- -A
led-roulette  :
section               size        addr
.vector_table          392   0x8000000
.text                16500   0x8000188
.rodata               2812   0x8004200
.data                    0  0x20000000
.bss                     4  0x20000000
.debug_str          234583         0x0
.debug_abbrev        11028         0x0
.debug_info         216136         0x0
.debug_ranges        41704         0x0
.debug_macinfo          44         0x0
.debug_pubnames      22666         0x0
.debug_pubtypes      48842         0x0
.ARM.attributes         58         0x0
.debug_frame         62536         0x0
.debug_line          89927         0x0
.debug_loc             374         0x0
.debug_aranges         160         0x0
Total               747766

$ cargo size --target thumbv7em-none-eabihf --bin led-roulette --release -- -A
led-roulette  :
section              size        addr
.vector_table         392   0x8000000
.text                1948   0x8000188
.rodata                84   0x8000924
.data                   0  0x20000000
.bss                    4  0x20000000
.debug_str          22672         0x0
.debug_loc           6313         0x0
.debug_abbrev        1464         0x0
.debug_info         40426         0x0
.debug_ranges        3008         0x0
.debug_macinfo          1         0x0
.debug_pubnames      5621         0x0
.debug_pubtypes     10001         0x0
.ARM.attributes        54         0x0
.debug_frame          212         0x0
.debug_line          9090         0x0
.debug_aranges        160         0x0
Total              101450
```

> **NOTE** The Cargo project is already configured to build the release binary using LTO.

Know how to read this output? The `text` section contains the program instructions. It's around 2KB
in my case. On the other hand, the `data` and `bss` sections contain variables statically allocated
in RAM (`static` variables). A `static` variable is being used in `aux5::init`; that's why it shows 4
bytes of `bss`.

One final thing! We have been running our programs from within GDB but our programs don't depend on
GDB at all. You can confirm this be closing both GDB and OpenOCD and then resetting the board by
pressing the black button on the board. The LED roulette application will run without intervention
of GDB.
