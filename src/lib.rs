//! Fast, SIMD accelerated noise generation functions
//! with optional runtime feature detection.
//!
//! [Github Link](https://github.com/verpeteren/rust-simd-noise)
//!
//!## Features
//!
//!* AVX2 instruction and a non SIMD fallback
//!* Simplex noise, fractal brownian motion
//!* 1D, 2D, 3D
//!
//!# Examples
//!
//! ```rust
//! use cursednoise::*;
//!
//! // Get a block of 2d fbm noise with default settings, 100 x 100, with values scaled to the range [0,1]
//! let mut noise = [0.0; 100 * 100];
//! NoiseBuilder::fbm_2d(100, 100).generate_scaled(0.0,1.0, noise.as_mut_ptr());
//!
//! ```
//!
//! ```
#![no_std]
#![allow(unsafe_op_in_unsafe_fn)]
extern crate panic_abort;
extern crate simdeez;

mod dimensional_being;
pub mod intrinsics;
pub mod noise;
mod noise_builder;
mod noise_dimensions;
mod noise_helpers_32;
mod noise_helpers_64;
mod noise_type;
mod shared;

use simdeez::prelude::*;

use dimensional_being::DimensionalBeing;
pub use noise_builder::NoiseBuilder;
pub use noise_dimensions::NoiseDimensions;
pub use noise_type::NoiseType;
use simdeez::engines::avx2::Avx2;

pub const VECSIZE: usize = 64 * 32 * 16;


macro_rules! avxecute {
    ($call:stmt) => {
        unsafe {
            Avx2::invoke(#[inline(always)] || { $call })
        }
    };
}

pub fn get_1d_noise(noise_type: &NoiseType, noise: *mut f32) -> (f32, f32) {
    avxecute!(noise_helpers_32::get_1d_noise::<Avx2>(noise_type, noise))
}

pub fn get_2d_noise(noise_type: &NoiseType, noise: *mut f32) -> (f32, f32) {
    avxecute!(noise_helpers_32::get_2d_noise::<Avx2>(noise_type, noise))
}

pub fn get_3d_noise(noise_type: &NoiseType, noise: *mut f32) -> (f32, f32) {
    avxecute!(noise_helpers_32::get_3d_noise::<Avx2>(noise_type, noise))
}

pub fn get_1d_scaled_noise(noise_type: &NoiseType, noise: *mut f32) {
    avxecute!(get_scaled_noise::<Avx2, _>(noise_type, noise, get_1d_noise))
}

pub fn get_2d_scaled_noise(noise_type: &NoiseType, noise: *mut f32) {
    avxecute!(get_scaled_noise::<Avx2, _>(noise_type, noise, get_2d_noise))
}

pub fn get_3d_scaled_noise(noise_type: &NoiseType, noise: *mut f32) {
    avxecute!(get_scaled_noise::<Avx2, _>(noise_type, noise, get_3d_noise))
}

mod settings;
use crate::shared::get_scaled_noise;
pub use settings::{FbmSettings, GradientSettings, Settings, SimplexSettings};
