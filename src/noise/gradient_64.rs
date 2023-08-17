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

    let h_and_8 = ((h & S::Vi64::set1(8)).cmp_eq(S::setzero_epi64())).cast_f64();
    S::blendv_pd(S::sub_pd(S::setzero_pd(), v), v, h_and_8)
}

/// Generates a random gradient vector where one component is ±1 and the other is ±2.
///
/// This differs from Gustavson's gradients by having a constant magnitude, providing results that
/// are more consistent between directions.
#[inline(always)]
pub unsafe fn grad2<S: Simd>(seed: i64, hash: S::Vi64) -> [S::Vf64; 2] {
    let h = ((hash ^ S::Vi64::set1(seed)) & S::Vi64::set1(7));
    let mask = (S::Vi64::set1(4).cmp_gt(h)).cast_f64();
    let x_magnitude = S::blendv_pd(S::Vf64::set1(2.0), S::Vf64::set1(1.0), mask);
    let y_magnitude = S::blendv_pd(S::Vf64::set1(1.0), S::Vf64::set1(2.0), mask);

    let h_and_1 = ((h & S::Vi64::set1(1)).cmp_eq(S::setzero_epi64())).cast_f64();
    let h_and_2 = ((h & S::Vi64::set1(2)).cmp_eq(S::setzero_epi64())).cast_f64();

    let gx = S::blendv_pd(
        S::sub_pd(S::setzero_pd(), x_magnitude),
        x_magnitude,
        S::blendv_pd(h_and_2, h_and_1, mask),
    );
    let gy = S::blendv_pd(
        S::sub_pd(S::setzero_pd(), y_magnitude),
        y_magnitude,
        S::blendv_pd(h_and_1, h_and_2, mask),
    );
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
    let u = S::blendv_pd(y, x, h.l8);
    let v = S::blendv_pd(S::blendv_pd(z, x, h.h12_or_14), y, h.l4);
    let result = S::add_pd((u ^ h.h1), (v ^ h.h2));
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
    gy = S::blendv_pd(gy, second, h.l4);
    gx = S::blendv_pd(gx, second, h.l4.and_not(h.h12_or_14));
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
    let u = S::blendv_pd(y, x, mask);
    mask = (S::Vi64::set1(16).cmp_gt(h)).cast_f64();
    let v = S::blendv_pd(z, y, mask);
    mask = (S::Vi64::set1(8).cmp_gt(h)).cast_f64();
    let w = S::blendv_pd(t, z, mask);

    let h_and_1 = ((h & S::Vi64::set1(1)).cmp_eq(S::setzero_epi64())).cast_f64();
    let h_and_2 = ((h & S::Vi64::set1(2)).cmp_eq(S::setzero_epi64())).cast_f64();
    let h_and_4 = ((h & S::Vi64::set1(4)).cmp_eq(S::setzero_epi64())).cast_f64();

    S::add_pd(
        S::blendv_pd(S::sub_pd(S::setzero_pd(), u), u, h_and_1),
        S::add_pd(
            S::blendv_pd(S::sub_pd(S::setzero_pd(), v), v, h_and_2),
            S::blendv_pd(S::sub_pd(S::setzero_pd(), w), w, h_and_4),
        ),
    )
}
