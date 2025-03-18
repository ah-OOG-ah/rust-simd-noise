use cursednoise::intrinsics::avx2;
use cursednoise::{GradientSettings, NoiseDimensions, Settings, SimplexSettings};

mod helpers;
use helpers::{read_from_file_f32, read_from_file_f64, BIN_PATH};

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_gradient_1_avx2_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        ..NoiseDimensions::default(1)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = avx2::get_1d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_1_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "1d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_1_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_gradient_1_avx2_64_normal() -> Vec<f64> {
    let dims = NoiseDimensions {
        width: 64,
        ..NoiseDimensions::default(1)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = avx2::get_1d_noise_64::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_1_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "1d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_1_avx2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_gradient_2_avx2_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_2_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "2d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_2_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_gradient_2_avx2_64_normal() -> Vec<f64> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = avx2::get_2d_noise_64::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_2_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "2d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_2_avx2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_gradient_3_avx2_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
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
unsafe fn do_intrinsic_gradient_3_avx2_64_normal() -> Vec<f64> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = avx2::get_3d_noise_64::<simdeez::Avx2>(&noise_type);
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