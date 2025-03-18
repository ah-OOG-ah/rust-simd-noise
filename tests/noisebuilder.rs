use cursednoise::{NoiseBuilder, Settings, SimplexSettings};

mod helpers;
use helpers::{read_from_file_f32, BIN_PATH};

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
                    let (noise, _min, _max) = NoiseBuilder::fbm_1d(64)
                        .with_freq(0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_fbm_nooffset_f32_2d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "nooffset", "32", "2d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::fbm_2d(64, 32)
                        .with_freq_2d(0.04, 0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

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
                    let (noise, _min, _max) = NoiseBuilder::fbm_3d(64, 32, 16)
                        .with_freq_3d(0.05, 0.04, 0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

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
                    let (noise, _min, _max) = NoiseBuilder::fbm_1d_offset(16.0, 64)
                        .with_freq(0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

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
                    let (noise, _min, _max) = NoiseBuilder::fbm_2d_offset(16.0, 64, 32.0, 32)
                        .with_freq_2d(0.04, 0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

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
                    let (noise, _min, _max) =
                        NoiseBuilder::fbm_3d_offset(16.0, 64, 32.0, 32, 64.0, 16)
                            .with_freq_3d(0.05, 0.04, 0.01)
                            .with_seed(1337)
                            .with_octaves(5)
                            .with_gain(2.0)
                            .with_lacunarity(0.5)
                            .generate();

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
                    let (noise, _min, _max) = NoiseBuilder::gradient_1d(64)
                        .with_freq(0.01)
                        .with_seed(1337)
                        .generate();

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
                    let (noise, _min, _max) = NoiseBuilder::gradient_2d(64, 32)
                        .with_freq_2d(0.04, 0.01)
                        .with_seed(1337)
                        .generate();

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
                    let (noise, _min, _max) = NoiseBuilder::gradient_3d(64, 32, 16)
                        .with_freq_3d(0.05, 0.04, 0.01)
                        .with_seed(1337)
                        .generate();

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
                    let (noise, _min, _max) = NoiseBuilder::gradient_1d_offset(16.0, 64)
                        .with_freq(0.01)
                        .with_seed(1337)
                        .generate();

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
                    let (noise, _min, _max) = NoiseBuilder::gradient_2d_offset(16.0, 64, 32.0, 32)
                        .with_freq_2d(0.04, 0.01)
                        .with_seed(1337)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_gradient_offset_f32_3d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "offset", "32", "3d"
                    );
                    let (noise, _min, _max) =
                        NoiseBuilder::gradient_3d_offset(16.0, 64, 32.0, 32, 64.0, 16)
                            .with_freq_3d(0.05, 0.04, 0.01)
                            .with_seed(1337)
                            .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
            }
        }
    }
}
