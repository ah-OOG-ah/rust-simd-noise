use super::{
    DimensionalBeing, FbmSettings, GradientSettings,
    NoiseDimensions,
};

/// Specifies what type of noise to generate and contains any relevant settings.
#[derive(Copy, Clone)]
pub enum NoiseType {
    Fbm(FbmSettings),
    Gradient(GradientSettings),
}

impl DimensionalBeing for NoiseType {
    fn get_dimensions(&self) -> NoiseDimensions {
        match self {
            NoiseType::Fbm(s) => s.get_dimensions(),
            NoiseType::Gradient(s) => s.get_dimensions(),
        }
    }
}
