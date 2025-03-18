use simdeez::prelude::*;

use super::NoiseType;
use crate::dimensional_being::DimensionalBeing;

use crate::{FbmSettings, GradientSettings, Settings};

use std::f64;

pub trait Sample64<S: Simd>: DimensionalBeing + Settings {
    fn sample_1d(&self, x: S::Vf64) -> S::Vf64;
    fn sample_2d(&self, x: S::Vf64, y: S::Vf64) -> S::Vf64;
    fn sample_3d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64) -> S::Vf64;
}

#[inline(always)]
unsafe fn get_1d_noise_helper_f64<S: Simd, Settings: Sample64<S>>(
    settings: Settings,
) -> (Vec<f64>, f64, f64) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf64::set1(settings.get_freq_x() as f64);
    let start_x = dim.x as f64;
    let width = dim.width;
    let mut min_s = S::Vf64::set1(f64::MAX);
    let mut max_s = S::Vf64::set1(f64::MIN);

    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let mut result = Vec::<f64>::with_capacity(width);
    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf64::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);
    let mut x = S::Vf64::load_from_ptr_unaligned(&x_arr[0]);
    for _ in 0..width / vector_width {
        let f = settings.sample_1d(x * freq_x);
        max_s = max_s.max(f);
        min_s = min_s.min(f);
        f.copy_to_ptr_unaligned(result_ptr.add(i));
        i += vector_width;
        x = x + S::Vf64::set1(vector_width as f64);
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
    result.set_len(width);
    for i in 0..vector_width {
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
unsafe fn get_2d_noise_helper_f64<S: Simd, Settings: Sample64<S>>(
    settings: Settings,
) -> (Vec<f64>, f64, f64) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf64::set1(settings.get_freq_x() as f64);
    let freq_y = S::Vf64::set1(settings.get_freq_y() as f64);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;

    let mut min_s = S::Vf64::set1(f64::MAX);
    let mut max_s = S::Vf64::set1(f64::MIN);
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let mut result = Vec::<f64>::with_capacity(width * height);
    let result_ptr = result.as_mut_ptr();
    let mut y = S::Vf64::set1(start_y);
    let mut i = 0;
    let vector_width = S::Vf64::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);
    for _ in 0..height {
        let mut x = S::Vf64::load_from_ptr_unaligned(&x_arr[0]);
        for _ in 0..width / vector_width {
            let f = settings.sample_2d(x * freq_x, y * freq_y);
            max_s = max_s.max(f);
            min_s = min_s.min(f);
            f.copy_to_ptr_unaligned(result_ptr.add(i));
            i += vector_width;
            x = x + S::Vf64::set1(vector_width as f64);
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
        y = y + S::Vf64::set1(1.0);
    }
    result.set_len(width * height);
    for i in 0..vector_width {
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
unsafe fn get_3d_noise_helper_f64<S: Simd, Settings: Sample64<S>>(
    settings: Settings,
) -> (Vec<f64>, f64, f64) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf64::set1(settings.get_freq_x() as f64);
    let freq_y = S::Vf64::set1(settings.get_freq_y() as f64);
    let freq_z = S::Vf64::set1(settings.get_freq_z() as f64);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;
    let start_z = dim.z as f64;
    let depth = dim.depth;

    let mut min_s = S::Vf64::set1(f64::MAX);
    let mut max_s = S::Vf64::set1(f64::MIN);
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let mut result = Vec::<f64>::with_capacity(width * height * depth);
    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf64::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);

    let mut z = S::Vf64::set1(start_z);
    for _ in 0..depth {
        let mut y = S::Vf64::set1(start_y);
        for _ in 0..height {
            let mut x = S::Vf64::load_from_ptr_unaligned(&x_arr[0]);
            for _ in 0..width / vector_width {
                let f = settings.sample_3d(x * freq_x, y * freq_y, z * freq_z);
                max_s = max_s.max(f);
                min_s = min_s.min(f);
                f.copy_to_ptr_unaligned(result_ptr.add(i));
                i += vector_width;
                x = x + S::Vf64::set1(vector_width as f64);
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
            y = y + S::Vf64::set1(1.0);
        }
        z = z + S::Vf64::set1(1.0);
    }
    result.set_len(width * height * depth);
    for i in 0..vector_width {
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
pub unsafe fn get_1d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_1d_noise_helper_f64::<S, FbmSettings>(*s),
        NoiseType::Gradient(s) => get_1d_noise_helper_f64::<S, GradientSettings>(*s),
    }
}

/// Gets a width X height sized block of 2d noise, unscaled.
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_2d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_2d_noise_helper_f64::<S, FbmSettings>(*s),
        NoiseType::Gradient(s) => get_2d_noise_helper_f64::<S, GradientSettings>(*s),
    }
}

/// Gets a width X height X depth sized block of 3d noise, unscaled,
/// `start_x`,`start_y` and `start_z` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_3d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_3d_noise_helper_f64::<S, FbmSettings>(*s),
        NoiseType::Gradient(s) => get_3d_noise_helper_f64::<S, GradientSettings>(*s),
    }
}