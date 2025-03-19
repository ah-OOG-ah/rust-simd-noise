use crate::dimensional_being::DimensionalBeing;
use crate::{FbmSettings, GradientSettings, NoiseType, Settings, VECSIZE};

use simdeez::prelude::*;

use core::f32;
use simdeez::engines::avx2::Avx2;

const VEC_WIDTH: usize = <Avx2 as simdeez::Simd>::Vf32::WIDTH;

pub trait Sample32<S: Simd>: DimensionalBeing + Settings {
    fn sample_1d(&self, x: S::Vf32) -> S::Vf32;
    fn sample_2d(&self, x: S::Vf32, y: S::Vf32) -> S::Vf32;
    fn sample_3d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32) -> S::Vf32;
}

#[inline(always)]
unsafe fn get_1d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
) -> ([f32; VECSIZE], f32, f32) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf32::set1(settings.get_freq_x());
    let start_x = dim.x;
    let width = dim.width;
    let mut min_s = S::Vf32::set1(f32::MAX);
    let mut max_s = S::Vf32::set1(f32::MIN);

    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = [0.0; VECSIZE];
    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let remainder = width % VEC_WIDTH;
    let mut x_arr = [0.0; VEC_WIDTH];
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..VEC_WIDTH).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    
    let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
    for _ in 0..width / VEC_WIDTH {
        let f = settings.sample_1d(x * freq_x);
        max_s = max_s.max(f);
        min_s = min_s.min(f);
        f.copy_to_ptr_unaligned(result_ptr.add(i));
        i += VEC_WIDTH;
        x = x + S::Vf32::set1(VEC_WIDTH as f32);
    }
    if remainder != 0 {
        let f = settings.sample_1d(x * freq_x);
        for j in 0..remainder {
            let n = f[j];
            result_ptr.add(i).write(n);
            // Note: This is unecessary for large images
            if n < min {
                min = n;
            }
            if n > max {
                max = n;
            }
            i += 1;
        }
    }
    
    for i in 0..VEC_WIDTH {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (result, min, max)
}

#[inline(always)]
unsafe fn get_2d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
) -> ([f32; VECSIZE], f32, f32) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf32::set1(settings.get_freq_x());
    let freq_y = S::Vf32::set1(settings.get_freq_y());
    let start_x = dim.x;
    let width = dim.width;
    let start_y = dim.y;
    let height = dim.height;

    let mut min_s = S::Vf32::set1(f32::MAX);
    let mut max_s = S::Vf32::set1(f32::MIN);
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = [0.0; VECSIZE];
    let result_ptr = result.as_mut_ptr();
    let mut y = S::Vf32::set1(start_y);
    let mut i = 0;
    let remainder = width % VEC_WIDTH;
    let mut x_arr = [0.0; VEC_WIDTH];
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..VEC_WIDTH).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    
    for _ in 0..height {
        let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
        for _ in 0..width / VEC_WIDTH {
            let f = settings.sample_2d(x * freq_x, y * freq_y);
            max_s = max_s.max(f);
            min_s = min_s.min(f);
            f.copy_to_ptr_unaligned(result_ptr.add(i));
            i += VEC_WIDTH;
            x = x + S::Vf32::set1(VEC_WIDTH as f32);
        }
        if remainder != 0 {
            let f = settings.sample_2d(x * freq_x, y * freq_y);
            for j in 0..remainder {
                let n = f[j];
                result_ptr.add(i).write(n);
                if n < min {
                    min = n;
                }
                if n > max {
                    max = n;
                }
                i += 1;
            }
        }
        y = y + S::Vf32::set1(1.0);
    }
    
    for i in 0..VEC_WIDTH {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (result, min, max)
}

#[inline(always)]
unsafe fn get_3d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
) -> ([f32; VECSIZE], f32, f32) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf32::set1(settings.get_freq_x());
    let freq_y = S::Vf32::set1(settings.get_freq_y());
    let freq_z = S::Vf32::set1(settings.get_freq_z());
    let start_x = dim.x;
    let width = dim.width;
    let start_y = dim.y;
    let height = dim.height;
    let start_z = dim.z;
    let depth = dim.depth;

    let mut min_s = S::Vf32::set1(f32::MAX);
    let mut max_s = S::Vf32::set1(f32::MIN);
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = [0.0; VECSIZE];
    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let remainder = width % VEC_WIDTH;
    let mut x_arr = [0.0; VEC_WIDTH];
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..VEC_WIDTH).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }

    let mut z = S::Vf32::set1(start_z);
    for _ in 0..depth {
        let mut y = S::Vf32::set1(start_y);
        for _ in 0..height {
            let mut x = S::Vf32::load_from_ptr_unaligned(&x_arr[0]);
            for _ in 0..width / VEC_WIDTH {
                let f = settings.sample_3d(x * freq_x, y * freq_y, z * freq_z);
                max_s = max_s.max(f);
                min_s = min_s.min(f);
                f.copy_to_ptr_unaligned(result_ptr.add(i));
                i += VEC_WIDTH;
                x = x + S::Vf32::set1(VEC_WIDTH as f32);
            }
            if remainder != 0 {
                let f = settings.sample_3d(x * freq_x, y * freq_y, z * freq_z);
                for j in 0..remainder {
                    let n = f[j];
                    result_ptr.add(i).write(n);
                    if n < min {
                        min = n;
                    }
                    if n > max {
                        max = n;
                    }
                    i += 1;
                }
            }
            y = y + S::Vf32::set1(1.0);
        }
        z = z + S::Vf32::set1(1.0);
    }

    for i in 0..VEC_WIDTH {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (result, min, max)
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_1d_noise<S: Simd>(noise_type: &NoiseType) -> ([f32; VECSIZE], f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_1d_noise_helper_f32::<S, FbmSettings>(*s),
        NoiseType::Gradient(s) => get_1d_noise_helper_f32::<S, GradientSettings>(*s),
    }
}

/// Gets a width X height sized block of 2d noise, unscaled.
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_2d_noise<S: Simd>(noise_type: &NoiseType) -> ([f32; VECSIZE], f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_2d_noise_helper_f32::<S, FbmSettings>(*s),
        NoiseType::Gradient(s) => get_2d_noise_helper_f32::<S, GradientSettings>(*s),
    }
}

/// Gets a width X height X depth sized block of 3d noise, unscaled,
/// `start_x`,`start_y` and `start_z` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_3d_noise<S: Simd>(noise_type: &NoiseType) -> ([f32; VECSIZE], f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_3d_noise_helper_f32::<S, FbmSettings>(*s),
        NoiseType::Gradient(s) => get_3d_noise_helper_f32::<S, GradientSettings>(*s),
    }
}