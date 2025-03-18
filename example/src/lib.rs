#![no_std]

use libc_print::std_name::{print, println};
use cursednoise::{NoiseBuilder, Settings, SimplexSettings};

fn main() {

    let AAAA = NoiseBuilder::fbm_3d(3, 3, 3).with_freq(0.01).with_seed(4002).with_octaves(3).generate_scaled(0.0, 5.0);

    for i in 0..3 {
        for ii in 0..9 {
            print!("{}, ", AAAA[i * 9 + ii])
        }
        println!()
    }
}