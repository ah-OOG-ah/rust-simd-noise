//! Noise functions that compute one value at a time
//!
//! These are much slower than SIMD, and hence on capable hardware have little use but testing.
use crate::noise::fbm_32;
use crate::noise::fbm_64;
use crate::noise::simplex_32;
use crate::noise::simplex_64;
use crate::noise_helpers_32;
use crate::noise_helpers_64;
use crate::{DimensionalBeing, NoiseType};

use crate::shared::scale_noise;

use simdeez::{SimdTransmuteF32, SimdTransmuteF64};

use core::f32;

simplex!(
    "1d",
    simplex_1d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    simplex_32,
    try_transmute_scalar
);
simplex!(
    "2d",
    simplex_2d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    simplex_32,
    try_transmute_scalar
);
simplex!(
    "3d",
    simplex_3d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    simplex_32,
    try_transmute_scalar
);
simplex!(
    "1d",
    simplex_1d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    simplex_64,
    try_transmute_scalar
);
simplex!(
    "2d",
    simplex_2d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    simplex_64,
    try_transmute_scalar
);
simplex!(
    "3d",
    simplex_3d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    simplex_64,
    try_transmute_scalar
);
fbm!(
    "1d",
    fbm_1d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    fbm_32,
    try_transmute_scalar
);
fbm!(
    "2d",
    fbm_2d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    fbm_32,
    try_transmute_scalar
);
fbm!(
    "3d",
    fbm_3d,
    f32,
    SimdTransmuteF32::try_transmute_from_scalar,
    i32,
    fbm_32,
    try_transmute_scalar
);
fbm!(
    "1d",
    fbm_1d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    fbm_64,
    try_transmute_scalar
);
fbm!(
    "2d",
    fbm_2d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    fbm_64,
    try_transmute_scalar
);
fbm!(
    "3d",
    fbm_3d_f64,
    f64,
    SimdTransmuteF64::try_transmute_from_scalar,
    i64,
    fbm_64,
    try_transmute_scalar
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