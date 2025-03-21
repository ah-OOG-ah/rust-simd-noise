#![allow(unsafe_op_in_unsafe_fn)]
use cursednoise::intrinsics::avx2;
use cursednoise::{GradientSettings, NoiseDimensions, Settings};

mod helpers;
use helpers::{read_from_file_f32, read_from_file_f64, BIN_PATH};
use simdeez::engines::avx2::Avx2;

const W: usize = 64;
const H: usize = 32;
const D: usize = 16;

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_gradient_1_avx2_32_normal() -> [f32; W] {
    let dims = NoiseDimensions {
        width: W,
        ..NoiseDimensions::default(1)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let mut noise = [0.0; W];
    let (_min, _max) = avx2::get_1d_noise::<Avx2>(&noise_type, noise.as_mut_ptr());
    noise
}

#[test]
fn test_intrinsic_gradient_1_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "1d", "avx2", "normal"
    );
    unsafe {
        let expected = read_from_file_f32(&file_name).unwrap();
        let noise = &do_intrinsic_gradient_1_avx2_32_normal()[0..expected.len()];
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_gradient_1_avx2_64_normal() -> [f64; W] {
    let dims = NoiseDimensions {
        width: W,
        ..NoiseDimensions::default(1)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let mut noise = [0.0; W];
    let (_min, _max) = avx2::get_1d_noise_64::<Avx2>(&noise_type, noise.as_mut_ptr());
    noise
}

#[test]
fn test_intrinsic_gradient_1_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "1d", "avx2", "normal"
    );
    unsafe {
        let expected = read_from_file_f64(&file_name).unwrap();
        let noise = &do_intrinsic_gradient_1_avx2_64_normal()[0..expected.len()];
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_gradient_2_avx2_32_normal() -> [f32; W * H] {
    let dims = NoiseDimensions {
        width: W,
        height: H,
        ..NoiseDimensions::default(2)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let mut noise = [0.0; W * H];
    let (_min, _max) = avx2::get_2d_noise::<Avx2>(&noise_type, noise.as_mut_ptr());
    noise
}

#[test]
fn test_intrinsic_gradient_2_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "2d", "avx2", "normal"
    );
    unsafe {
        let expected = read_from_file_f32(&file_name).unwrap();
        let noise = &do_intrinsic_gradient_2_avx2_32_normal()[0..expected.len()];
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_gradient_2_avx2_64_normal() -> [f64; W * H] {
    let dims = NoiseDimensions {
        width: W,
        height: H,
        ..NoiseDimensions::default(2)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let mut noise = [0.0; W * H];
    let (_min, _max) = avx2::get_2d_noise_64::<Avx2>(&noise_type, noise.as_mut_ptr());
    noise
}

#[test]
fn test_intrinsic_gradient_2_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "2d", "avx2", "normal"
    );
    unsafe {
        let expected = read_from_file_f64(&file_name).unwrap();
        let noise = &do_intrinsic_gradient_2_avx2_64_normal()[0..expected.len()];
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_gradient_3_avx2_32_normal() -> [f32; W * H * D] {
    let dims = NoiseDimensions {
        width: W,
        height: H,
        depth: D,
        ..NoiseDimensions::default(3)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let mut noise = [0.0; W * H * D];
    let (_min, _max) = avx2::get_3d_noise::<Avx2>(&noise_type, noise.as_mut_ptr());
    noise
}

#[test]
fn test_intrinsic_gradient_3_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "3d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_3_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_gradient_3_avx2_64_normal() -> [f64; W * H * D] {
    let dims = NoiseDimensions {
        width: W,
        height: H,
        depth: D,
        ..NoiseDimensions::default(3)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let mut noise = [0.0; W * H * D];
    let (_min, _max) = avx2::get_3d_noise_64::<Avx2>(&noise_type, noise.as_mut_ptr());
    noise
}

#[test]
#[should_panic(expected = "not implemented")]
fn test_intrinsic_gradient_3_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "3d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_3_avx2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}