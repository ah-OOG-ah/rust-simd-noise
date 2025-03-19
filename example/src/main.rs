#![no_std]
#![no_main]
#![allow(clippy::needless_return)]

use core::panic::PanicInfo;
use libc_print::std_name::{print, println};
use cursednoise::{NoiseBuilder, Settings, SimplexSettings};

#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize{

    let AAAA = NoiseBuilder::fbm_3d(3, 3, 3).with_freq(0.01).with_seed(4002).with_octaves(3).generate_scaled(0.0, 5.0);

    for i in 0..3 {
        for ii in 0..9 {
            print!("{}, ", AAAA[i * 9 + ii])
        }
        println!()
    }

    return 0;
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}