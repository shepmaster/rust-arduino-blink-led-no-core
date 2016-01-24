#![feature(no_core)]
#![feature(lang_items)]
#![feature(fundamental)]
#![feature(intrinsics)]
#![feature(on_unimplemented)]
#![feature(optin_builtin_traits)]
#![feature(reflect)]
#![feature(unboxed_closures)]
#![feature(associated_type_defaults)]
#![feature(asm)]

#![no_core]
#![no_main]

const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;

fn sleep_ms(duration_ms: u16) {
    const FREQUENCY_HZ: u32 = 16_000_000;
    const CYCLES_PER_MS: u16 = (FREQUENCY_HZ / 1000) as u16;
    const CYCLES_PER_INNER_LOOP: u16 = 6; // From the disassembly
    const INNER_LOOP_ITERATIONS: u16 = CYCLES_PER_MS / CYCLES_PER_INNER_LOOP;

    let mut outer = 0;
    while outer < duration_ms {
        let mut inner = 0;
        while inner < INNER_LOOP_ITERATIONS {
            unsafe { asm!(""); }
            inner += 1;
        }
        outer += 1;
    }
}

#[no_mangle]
pub extern fn main() {
    unsafe {
        volatile_store(DDRB, 0xFF); // Everything is set to output
        loop {
            sleep_ms(500);
            volatile_store(PORTB, 0xFF); // Everything is on
            sleep_ms(500);
            volatile_store(PORTB, 0x00); // Everything is off
        }
    }
}

// All this is copied from libcore

use option::Option::*;
use intrinsics::{volatile_store, volatile_load};
use ops::*;

#[lang = "panic"]
pub fn panic(expr_file_line: &(&'static str, &'static str, u32)) -> ! {
    loop {}
}

mod option;
mod intrinsics;
mod clone;
mod marker;
mod cmp;
mod ops;
