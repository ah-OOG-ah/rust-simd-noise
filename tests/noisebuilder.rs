use cursednoise::{NoiseBuilder, Settings, SimplexSettings};

mod helpers;
use helpers::{read_from_file_f32, BIN_PATH};

const W: usize = 64;
const H: usize = 32;
const D: usize = 16;

mod noise {
    use super::*;

    mod fbm {
        use super::*;
        mod f32 {
            use super::*;

            mod nooffset {
                use super::*;

                #[test]
                fn test_noisebuilder_fbm_nooffset_f32_1d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "nooffset", "32", "1d"
                    );
                    let mut noise = [0.0; W];
                    let (_min, _max) = NoiseBuilder::fbm_1d(W)
                        .with_freq(0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate(noise.as_mut_ptr());

                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_fbm_nooffset_f32_2d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "nooffset", "32", "2d"
                    );
                    let mut noise = [0.0; W * H];
                    let (_min, _max) = NoiseBuilder::fbm_2d(W, H)
                        .with_freq_2d(0.04, 0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate(noise.as_mut_ptr());

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_fbm_nooffset_f32_3d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "nooffset", "32", "3d"
                    );
                    let mut noise = [0.0; W * H * D];
                    let (_min, _max) = NoiseBuilder::fbm_3d(W, H, D)
                        .with_freq_3d(0.05, 0.04, 0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate(noise.as_mut_ptr());

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
            }

            mod offset {
                use super::*;

                #[test]
                fn test_noisebuilder_fbm_offset_f32_1d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "offset", "32", "1d"
                    );
                    let mut noise = [0.0; W];
                    let (_min, _max) = NoiseBuilder::fbm_1d_offset(16.0, W)
                        .with_freq(0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate(noise.as_mut_ptr());

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_fbm_offset_f32_2d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "offset", "32", "2d"
                    );
                    let mut noise = [0.0; W * H];
                    let (_min, _max) = NoiseBuilder::fbm_2d_offset(16.0, W, 32.0, H)
                        .with_freq_2d(0.04, 0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate(noise.as_mut_ptr());

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_fbm_offset_f32_3d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "offset", "32", "3d"
                    );
                    let mut noise = [0.0; W * H * D];
                    let (_min, _max) =
                        NoiseBuilder::fbm_3d_offset(16.0, W, 32.0, H, 64.0, D)
                            .with_freq_3d(0.05, 0.04, 0.01)
                            .with_seed(1337)
                            .with_octaves(5)
                            .with_gain(2.0)
                            .with_lacunarity(0.5)
                            .generate(noise.as_mut_ptr());

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
            }
        }
    }
    mod gradient {
        use super::*;
        mod f32 {
            use super::*;

            mod nooffset {
                use super::*;

                #[test]
                fn test_noisebuilder_gradient_nooffset_f32_1d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "nooffset", "32", "1d"
                    );
                    let mut noise = [0.0; W];
                    let (_min, _max) = NoiseBuilder::gradient_1d(W)
                        .with_freq(0.01)
                        .with_seed(1337)
                        .generate(noise.as_mut_ptr());

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_gradient_nooffset_f32_2d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "nooffset", "32", "2d"
                    );
                    let mut noise = [0.0; W * H];
                    let (_min, _max) = NoiseBuilder::gradient_2d(W, H)
                        .with_freq_2d(0.04, 0.01)
                        .with_seed(1337)
                        .generate(noise.as_mut_ptr());

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_gradient_nooffset_f32_3d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "nooffset", "32", "3d"
                    );
                    let mut noise = [0.0; W * H * D];
                    let (_min, _max) = NoiseBuilder::gradient_3d(W, H, D)
                        .with_freq_3d(0.05, 0.04, 0.01)
                        .with_seed(1337)
                        .generate(noise.as_mut_ptr());

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
            }

            mod offset {
                use super::*;

                #[test]
                fn test_noisebuilder_gradient_offset_f32_1d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "offset", "32", "1d"
                    );
                    let mut noise = [0.0; W];
                    let (_min, _max) = NoiseBuilder::gradient_1d_offset(16.0, W)
                        .with_freq(0.01)
                        .with_seed(1337)
                        .generate(noise.as_mut_ptr());

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_gradient_offset_f32_2d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "offset", "32", "2d"
                    );
                    let mut noise = [0.0; W * H];
                    let (_min, _max) = NoiseBuilder::gradient_2d_offset(16.0, W, 32.0, H)
                        .with_freq_2d(0.04, 0.01)
                        .with_seed(1337)
                        .generate(noise.as_mut_ptr());

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, &noise[0..expected.len()]);
                }
                #[test]
                fn test_noisebuilder_gradient_offset_f32_3d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "offset", "32", "3d"
                    );
                    let mut noise = [0.0; W * H * D];
                    let (_min, _max) =
                        NoiseBuilder::gradient_3d_offset(16.0, W, 32.0, H, 64.0, D)
                            .with_freq_3d(0.05, 0.04, 0.01)
                            .with_seed(1337)
                            .generate(noise.as_mut_ptr());

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
            }
        }
    }
}
