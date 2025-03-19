use simdeez::prelude::*;

use crate::{dimensional_being::DimensionalBeing, NoiseType};

#[inline(always)]
pub unsafe fn scale_noise<S: Simd>(
    scale_min: f32,
    scale_max: f32,
    min: f32,
    max: f32,
    data: *mut f32,
    len: usize
) {
    let scale_range = scale_max - scale_min;
    let range = max - min;
    let multiplier = scale_range / range;
    let offset = scale_min - min * multiplier;
    let vector_width = S::Vf32::WIDTH;
    let mut i = 0;
    if len >= vector_width {
        while i <= len - vector_width {
            let value = (S::Vf32::set1(multiplier) * S::Vf32::load_from_ptr_unaligned(data.add(i))) + S::Vf32::set1(offset);
            value.copy_to_ptr_unaligned(data.add(i));
            i += vector_width;
        }
    }
    i = len - (len % vector_width);
    while i < len {
        *data.add(i) = *data.add(i) * multiplier + offset;
        i += 1;
    }
}

pub(crate) unsafe fn get_scaled_noise<S: Simd, F: Fn(&NoiseType, *mut f32) -> (f32, f32)>(noise_type: &NoiseType, noise: *mut f32, noise_fn: F) {
    let (min, max) = noise_fn(noise_type, noise);
    let dim = noise_type.get_dimensions();
    scale_noise::<S>(dim.min, dim.max, min, max, noise, dim.len());
}
