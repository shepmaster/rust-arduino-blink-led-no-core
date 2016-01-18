#![feature(no_core)]
#![feature(lang_items)]
#![feature(fundamental)]
#![feature(intrinsics)]
#![feature(on_unimplemented)]
#![feature(optin_builtin_traits)]
#![feature(reflect)]
#![feature(unboxed_closures)]
#![feature(associated_type_defaults)]

#![no_core]
#![no_main]

const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;

#[no_mangle]
pub extern fn main() {
    unsafe {
        volatile_store(DDRB, 0xFF); // Everything is set to output
        loop {
            volatile_store(PORTB, 0xFF); // Everything is on
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
