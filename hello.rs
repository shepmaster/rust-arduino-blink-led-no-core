#![feature(no_core)]
#![feature(lang_items)]
#![feature(fundamental)]
#![feature(intrinsics)]

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

extern "rust-intrinsic" {
    pub fn volatile_load<T>(src: *const T) -> T;
    pub fn volatile_store<T>(src: *mut T, value: T);
}

#[lang = "sized"]
#[fundamental]
pub trait Sized {}

#[lang = "copy"]
pub trait Copy : Clone {}

pub trait Clone : Sized {
    fn clone(&self) -> Self;

    #[inline(always)]
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}
