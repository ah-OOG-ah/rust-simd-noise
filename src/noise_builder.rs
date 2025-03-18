use super::{
    FbmSettings, GradientSettings, NoiseDimensions
    , Settings,
};

pub struct NoiseBuilder {}

impl NoiseBuilder {

    pub fn fbm_1d(width: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.width = width;
        FbmSettings::default(dim)
    }

    pub fn fbm_1d_offset(x_offset: f32, width: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.width = width;
        dim.x = x_offset;
        FbmSettings::default(dim)
    }

    pub fn fbm_2d(width: usize, height: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        FbmSettings::default(dim)
    }

    pub fn fbm_2d_offset(x_offset: f32, width: usize, y_offset: f32, height: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        dim.x = x_offset;
        dim.y = y_offset;
        FbmSettings::default(dim)
    }

    pub fn fbm_3d(width: usize, height: usize, depth: usize) -> FbmSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        FbmSettings::default(dim)
    }

    pub fn fbm_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> FbmSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        FbmSettings::default(dim)
    }

    // Gradient Builders
    pub fn gradient_1d(width: usize) -> GradientSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.width = width;
        GradientSettings::default(dim)
    }

    pub fn gradient_1d_offset(x_offset: f32, width: usize) -> GradientSettings {
        let mut dim = NoiseDimensions::default(1);
        dim.width = width;
        dim.x = x_offset;
        GradientSettings::default(dim)
    }

    pub fn gradient_2d(width: usize, height: usize) -> GradientSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        GradientSettings::default(dim)
    }

    pub fn gradient_2d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
    ) -> GradientSettings {
        let mut dim = NoiseDimensions::default(2);
        dim.width = width;
        dim.height = height;
        dim.x = x_offset;
        dim.y = y_offset;
        GradientSettings::default(dim)
    }

    pub fn gradient_3d(width: usize, height: usize, depth: usize) -> GradientSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        GradientSettings::default(dim)
    }

    pub fn gradient_3d_offset(
        x_offset: f32,
        width: usize,
        y_offset: f32,
        height: usize,
        z_offset: f32,
        depth: usize,
    ) -> GradientSettings {
        let mut dim = NoiseDimensions::default(3);
        dim.width = width;
        dim.height = height;
        dim.depth = depth;
        dim.x = x_offset;
        dim.y = y_offset;
        dim.z = z_offset;
        GradientSettings::default(dim)
    }
}
