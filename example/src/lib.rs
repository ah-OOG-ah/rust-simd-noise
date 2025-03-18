use std::fmt::{Display, Formatter, Result};
use std::slice::Iter;
use cursednoise::{NoiseBuilder, Settings, SimplexSettings};

const FPS: usize = 60;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const DEPTH: usize = 1;
const TIME: usize = 5;
const DEFAULT_FREQUENCY: f32 = 1.2;
const DEFAULT_JITTER: f32 = 1.2;
const DEFAULT_LACUNARITY: f32 = 0.5;
const DEFAULT_GAIN: f32 = 2.0;
const DEFAULT_OCTAVES: u8 = 3;

const SCALE_MIN: f32 = 0.0;
const SCALE_MAX: f32 = 255.0;

fn main() {

    let AAAA = NoiseBuilder::fbm_3d(3, 3, 3).with_freq(0.01).with_seed(4002).with_octaves(3).generate_scaled(0.0, 5.0);

    for i in 0..3 {
        for ii in 0..9 {
            print!("{}, ", AAAA[i * 9 + ii])
        }
        println!()
    }
}