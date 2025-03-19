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
//! let noise =  NoiseBuilder::fbm_2d(100, 100).generate_scaled(0.0,1.0);
//!
//! ```
//!
//! ```
#![no_std]

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

use shared::get_scaled_noise;
use simdeez::fix_tuple_type;
use simdeez::prelude::*;

use dimensional_being::DimensionalBeing;
pub use noise_builder::NoiseBuilder;
pub use noise_dimensions::NoiseDimensions;
pub use noise_type::NoiseType;

pub const VECSIZE: usize = 5 * 33 * 5;

simd_compiletime_select!(
    pub fn get_1d_noise(noise_type: &NoiseType) -> ([f32; VECSIZE], f32, f32) {
        noise_helpers_32::get_1d_noise::<S>(noise_type)
    }
);

simd_compiletime_select!(
    pub fn get_2d_noise(noise_type: &NoiseType) -> ([f32; VECSIZE], f32, f32) {
        noise_helpers_32::get_2d_noise::<S>(noise_type)
    }
);

simd_compiletime_select!(
    pub fn get_3d_noise(noise_type: &NoiseType) -> ([f32; VECSIZE], f32, f32) {
        noise_helpers_32::get_3d_noise::<S>(noise_type)
    }
);

simd_compiletime_select!(
    pub fn get_1d_scaled_noise(noise_type: &NoiseType) -> [f32; VECSIZE] {
        unsafe { get_scaled_noise::<S, _>(noise_type, get_1d_noise) }
    }
);

simd_compiletime_select!(
    pub fn get_2d_scaled_noise(noise_type: &NoiseType) -> [f32; VECSIZE] {
        unsafe { get_scaled_noise::<S, _>(noise_type, get_2d_noise) }
    }
);

simd_compiletime_select!(
    pub fn get_3d_scaled_noise(noise_type: &NoiseType) -> [f32; VECSIZE] {
        unsafe { get_scaled_noise::<S, _>(noise_type, get_3d_noise) }
    }
);

mod settings;
pub use settings::{FbmSettings, GradientSettings, Settings, SimplexSettings};
