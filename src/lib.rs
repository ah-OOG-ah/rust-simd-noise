//! Fast, SIMD accelerated noise generation functions
//! with optional runtime feature detection.
//!
//! [Github Link](https://github.com/verpeteren/rust-simd-noise)
//!
//!## Features
//!
//!* SSE2, SSE41, and AVX2 instruction sets, along with non SIMD fallback
//!* Runtime detection picks the best available instruction set
//!* Simplex noise, fractal brownian motion, turbulence, and ridge
//!* 1D, 2D, 3D, and 4D
//!* Cellular / Voroni Noise  2D and 3D
//!
//!## Benchmarks
//! See [Github](https://github.com/verpeteren/rust-simd-noise)
//!## Todo
//!
//!* AVX512 support
//!* ARM NEON support
//!* Other noise types
//!
//!# Examples
//!
//!## Get a block of noise with runtime SIMD detection
//!
//! The library will, at runtime, pick the fastest available options between SSE2, SSE41, and AVX2
//!
//! ```rust
//! use simdnoise::*;
//!
//! // Get a block of 2d fbm noise with default settings, 100 x 100, with values scaled to the range [0,1]
//! let noise =  NoiseBuilder::fbm_2d(100, 100).generate_scaled(0.0,1.0);
//!
//! // Get a block of 4d ridge noise, custom settings, 32x32x32x32 unscaled
//! let (noise,min,max) =  NoiseBuilder::ridge_4d(32,32,32,32)
//!        .with_freq(0.05)
//!        .with_octaves(5)
//!        .with_gain(2.0)
//!        .with_seed(1337)
//!        .with_lacunarity(0.5)
//!        .generate();
//!
//! ```
//!
//! ## Call noise functions directly
//! Sometimes you need something other than a block, like the points on the surface of a sphere.
//! Sometimes you may want to use SSE41 even with AVX2 is available
//!
//!
#![cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), doc = "```rust")]
#![cfg_attr(
    not(any(target_arch = "x86", target_arch = "x86_64")),
    doc = "```rust,ignore"
)]
//! use simdnoise::*;
//! use core::arch::x86_64::*;
//!
//! let noise_setting =  NoiseBuilder::cellular2_3d(32,32,32)
//!         .with_freq(0.05)
//!         .with_return_type(Cell2ReturnType::Distance2Mul)
//!         .with_jitter(0.5)
//!         .wrap();
//!
//! // get a block of noise with the sse41 version, using the above settings
//! unsafe {
//!     let (noise,min,max) = simdnoise::intrinsics::sse41::get_3d_noise::<simdeez::Sse41>(&noise_setting);
//! }
//!
//! // send your own SIMD x,y values to the noise functions directly
//! unsafe {
//!   // sse2 simplex noise
//!   let x = _mm_set1_ps(5.0);
//!   let y = _mm_set1_ps(10.0);
//!   let seed = 42;
//!   let f : __m128 = simdnoise::intrinsics::sse2::simplex_2d::<simdeez::Sse2>(x,y,seed);
//!
//!   // avx2 turbulence
//!   let x = _mm256_set1_ps(5.0);
//!   let y = _mm256_set1_ps(10.0);
//!   let lacunarity = _mm256_set1_ps(0.5);
//!   let gain = _mm256_set1_ps(2.0);
//!   let octaves = 3;
//!   let f_turbulence : __m256 = simdnoise::intrinsics::avx2::turbulence_2d::<simdeez::Avx2>(x,y,lacunarity,gain,octaves,seed);
//!
//! }
//! ```

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
use simdeez::prelude::*;

use dimensional_being::DimensionalBeing;
pub use noise_builder::NoiseBuilder;
pub use noise_dimensions::NoiseDimensions;
pub use noise_type::NoiseType;

simd_runtime_generate!(
    pub fn get_1d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
        noise_helpers_32::get_1d_noise::<S>(noise_type)
    }
);

simd_runtime_generate!(
    pub fn get_2d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
        noise_helpers_32::get_2d_noise::<S>(noise_type)
    }
);

simd_runtime_generate!(
    pub fn get_3d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
        noise_helpers_32::get_3d_noise::<S>(noise_type)
    }
);

simd_runtime_generate!(
    pub fn get_1d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
        unsafe { get_scaled_noise::<S, _>(noise_type, get_1d_noise) }
    }
);

simd_runtime_generate!(
    pub fn get_2d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
        unsafe { get_scaled_noise::<S, _>(noise_type, get_2d_noise) }
    }
);

simd_runtime_generate!(
    pub fn get_3d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
        unsafe { get_scaled_noise::<S, _>(noise_type, get_3d_noise) }
    }
);

mod settings;
pub use settings::{FbmSettings, GradientSettings, Settings, SimplexSettings};