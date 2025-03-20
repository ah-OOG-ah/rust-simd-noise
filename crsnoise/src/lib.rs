#![no_main]
#![no_std]
#![allow(non_snake_case)]

use core::panic::PanicInfo;
use libc_print::std_name::{print, println};
use cursednoise::{NoiseBuilder, Settings, SimplexSettings};

#[unsafe(no_mangle)]
pub extern "C" fn generateNoise(
    noise: *mut f32, scale: f32, seed: i32,
    x0: f32, y0: f32, z0: f32,
    xLen: usize, yLen: usize, zLen: usize,
    xStep: f32, yStep: f32, zStep: f32) {
    NoiseBuilder::gradient_3d_offset(x0, xLen, y0, yLen, z0, zLen)
        .with_freq_3d(xStep, yStep, zStep)
        .with_seed(seed)
        .generate_scaled(0.0, scale, noise);
}

fn test() {
    let mut AAAA = [0.0; 3 * 3 * 3];
    NoiseBuilder::fbm_3d(3, 3, 3)
        .with_freq(0.01)
        .with_seed(4002)
        .with_octaves(3)
        .generate_scaled(0.0, 5.0, AAAA.as_mut_ptr());

    for i in 0..3 {
        for ii in 0..9 {
            print!("{}, ", AAAA[i * 9 + ii])
        }
        println!()
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    println!("rust panic!");
    loop {}
}