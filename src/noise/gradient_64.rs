use crate::noise::hash3d_64::hash3d;

use simdeez::prelude::*;

/// Generates a random integer gradient in ±7 inclusive
///
/// This differs from Gustavson's well-known implementation in that gradients can be zero, and the
/// maximum gradient is 7 rather than 8.
#[inline(always)]
pub unsafe fn grad1<S: Simd>(seed: i64, hash: S::Vi64) -> S::Vf64 {
    let h = ((S::Vi64::set1(seed) ^ hash) & S::Vi64::set1(15));
    let v = (h & S::Vi64::set1(7)).cast_f64();

    let h_and_8 = ((h & S::Vi64::set1(8)).cmp_eq(S::Vi64::zeroes())).cast_f64();
    h_and_8.blendv(S::sub_pd(S::Vf64::zeroes(), v), v)
}

/// Generates a random gradient vector where one component is ±1 and the other is ±2.
///
/// This differs from Gustavson's gradients by having a constant magnitude, providing results that
/// are more consistent between directions.
#[inline(always)]
pub unsafe fn grad2<S: Simd>(seed: i64, hash: S::Vi64) -> [S::Vf64; 2] {
    let h = ((hash ^ S::Vi64::set1(seed)) & S::Vi64::set1(7));
    let mask = (S::Vi64::set1(4).cmp_gt(h)).cast_f64();
    let x_magnitude = mask.blendv(S::Vf64::set1(2.0), S::Vf64::set1(1.0));
    let y_magnitude = mask.blendv(S::Vf64::set1(1.0), S::Vf64::set1(2.0));

    let h_and_1 = ((h & S::Vi64::set1(1)).cmp_eq(S::Vi64::zeroes())).cast_f64();
    let h_and_2 = ((h & S::Vi64::set1(2)).cmp_eq(S::Vi64::zeroes())).cast_f64();

    let gx = mask
        .blendv(h_and_2, h_and_1)
        .blendv(S::sub_pd(S::Vf64::zeroes(), x_magnitude), x_magnitude);
    let gy = mask
        .blendv(h_and_1, h_and_2)
        .blendv(S::sub_pd(S::Vf64::zeroes(), y_magnitude), y_magnitude);
    [gx, gy]
}

/// Generates a random gradient vector from the origin towards the midpoint of an edge of a
/// double-unit cube and computes its dot product with [x, y, z]
#[inline(always)]
pub unsafe fn grad3d_dot<S: Simd>(
    seed: i64,
    i: S::Vi64,
    j: S::Vi64,
    k: S::Vi64,
    x: S::Vf64,
    y: S::Vf64,
    z: S::Vf64,
) -> S::Vf64 {
    let h = hash3d::<S>(seed, i, j, k);
    let u = h.l8.blendv(y, x);
    let v = h.l4.blendv(h.h12_or_14.blendv(z, x), y);
    let result = (u ^ h.h1) + (v ^ h.h2);
    debug_assert_eq!(
        result[0],
        {
            let [gx, gy, gz] = grad3d::<S>(seed, i, j, k);
            gx * x + gy * y + gz * z
        }[0],
        "results match"
    );
    result
}

/// The gradient vector generated by `grad3d_dot`
///
/// This is a separate function because it's slower than `grad3d_dot` and only needed when computing
/// derivatives.
pub unsafe fn grad3d<S: Simd>(seed: i64, i: S::Vi64, j: S::Vi64, k: S::Vi64) -> [S::Vf64; 3] {
    let h = hash3d::<S>(seed, i, j, k);

    let first = S::Vf64::set1(1.0) | h.h1;
    let mut gx = h.l8 & first;
    let mut gy = h.l8.and_not(first);

    let second = S::Vf64::set1(1.0) | h.h2;
    gy = h.l4.blendv(gy, second);
    gx = h.l4.and_not(h.h12_or_14).blendv(gx, second);
    let gz = (h.h12_or_14 | h.l4).and_not(second);
    debug_assert_eq!(
        gx[0].abs() + gy[0].abs() + gz[0].abs(),
        2.0,
        "exactly two axes are chosen"
    );
    [gx, gy, gz]
}

#[inline(always)]
pub unsafe fn grad4<S: Simd>(
    seed: i64,
    hash: S::Vi64,
    x: S::Vf64,
    y: S::Vf64,
    z: S::Vf64,
    t: S::Vf64,
) -> S::Vf64 {
    let h = ((S::Vi64::set1(seed) ^ hash) & S::Vi64::set1(31));
    let mut mask = (S::Vi64::set1(24).cmp_gt(h)).cast_f64();
    let u = mask.blendv(y, x);
    mask = (S::Vi64::set1(16).cmp_gt(h)).cast_f64();
    let v = mask.blendv(z, y);
    mask = (S::Vi64::set1(8).cmp_gt(h)).cast_f64();
    let w = mask.blendv(t, z);

    let h_and_1 = ((h & S::Vi64::set1(1)).cmp_eq(S::Vi64::zeroes())).cast_f64();
    let h_and_2 = ((h & S::Vi64::set1(2)).cmp_eq(S::Vi64::zeroes())).cast_f64();
    let h_and_4 = ((h & S::Vi64::set1(4)).cmp_eq(S::Vi64::zeroes())).cast_f64();

    h_and_1.blendv(S::sub_pd(S::Vf64::zeroes(), u), u)
        + (h_and_2.blendv(S::sub_pd(S::Vf64::zeroes(), v), v)
            + h_and_4.blendv(S::sub_pd(S::Vf64::zeroes(), w), w),)
}
