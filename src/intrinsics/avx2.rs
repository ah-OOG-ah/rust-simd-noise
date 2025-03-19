//! AVX2 and FMA3 Accelerated noise functions.
//! CPUs since ~2013 (Intel) and ~2015 (AMD) support this.
//! It is about twice as fast as the SSE2 version.
//!
//! Use `is_x86_feature_detected!("avx2")` provided
//! by the Rust stanard library to detect at runtime.
//!
//! When using the `get_` functions, you will get a performance boost when width
//! is evenly divisble by 8, and when it is not small relative height and depth.

use crate::noise::fbm_32;
use crate::noise::fbm_64;
use crate::noise::simplex_32;
use crate::noise::simplex_64;
use crate::noise_helpers_32;
use crate::noise_helpers_64;
use crate::shared::scale_noise;
use crate::VECSIZE;
use crate::{DimensionalBeing, NoiseType};

use simdeez::{SimdTransmuteF32, SimdTransmuteF64};

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use core::f32;

simplex!(
    "1d",
    simplex_1d,
    __m256,
    SimdTransmuteF32::try_transmute_from_avx2,
    i32,
    simplex_32,
    try_transmute_avx2
);
simplex!(
    "2d",
    simplex_2d,
    __m256,
    SimdTransmuteF32::try_transmute_from_avx2,
    i32,
    simplex_32,
    try_transmute_avx2
);
simplex!(
    "3d",
    simplex_3d,
    __m256,
    SimdTransmuteF32::try_transmute_from_avx2,
    i32,
    simplex_32,
    try_transmute_avx2
);
simplex!(
    "1d",
    simplex_1d_f64,
    __m256d,
    SimdTransmuteF64::try_transmute_from_avx2,
    i64,
    simplex_64,
    try_transmute_avx2
);
simplex!(
    "2d",
    simplex_2d_f64,
    __m256d,
    SimdTransmuteF64::try_transmute_from_avx2,
    i64,
    simplex_64,
    try_transmute_avx2
);
simplex!(
    "3d",
    simplex_3d_f64,
    __m256d,
    SimdTransmuteF64::try_transmute_from_avx2,
    i64,
    simplex_64,
    try_transmute_avx2
);
fbm!(
    "1d",
    fbm_1d,
    __m256,
    SimdTransmuteF32::try_transmute_from_avx2,
    i32,
    fbm_32,
    try_transmute_avx2
);
fbm!(
    "2d",
    fbm_2d,
    __m256,
    SimdTransmuteF32::try_transmute_from_avx2,
    i32,
    fbm_32,
    try_transmute_avx2
);
fbm!(
    "3d",
    fbm_3d,
    __m256,
    SimdTransmuteF32::try_transmute_from_avx2,
    i32,
    fbm_32,
    try_transmute_avx2
);
fbm!(
    "1d",
    fbm_1d_f64,
    __m256d,
    SimdTransmuteF64::try_transmute_from_avx2,
    i64,
    fbm_64,
    try_transmute_avx2
);
fbm!(
    "2d",
    fbm_2d_f64,
    __m256d,
    SimdTransmuteF64::try_transmute_from_avx2,
    i64,
    fbm_64,
    try_transmute_avx2
);
fbm!(
    "3d",
    fbm_3d_f64,
    __m256d,
    SimdTransmuteF64::try_transmute_from_avx2,
    i64,
    fbm_64,
    try_transmute_avx2
);

get_noise!(get_1d_noise, get_1d_noise, f32, noise_helpers_32);
get_noise!(get_2d_noise, get_2d_noise, f32, noise_helpers_32);
get_noise!(get_3d_noise, get_3d_noise, f32, noise_helpers_32);
get_noise!(get_1d_noise, get_1d_noise_64, f64, noise_helpers_64);
get_noise!(get_2d_noise, get_2d_noise_64, f64, noise_helpers_64);
get_noise!(get_3d_noise, get_3d_noise_64, f64, noise_helpers_64);
get_noise_scaled!(get_1d_noise, get_1d_scaled_noise, f32);
get_noise_scaled!(get_2d_noise, get_2d_scaled_noise, f32);
get_noise_scaled!(get_3d_noise, get_3d_scaled_noise, f32);
