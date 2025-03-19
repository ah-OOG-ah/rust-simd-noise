#![allow(clippy::needless_return)]

#[derive(Copy, Clone)]
pub struct NoiseDimensions {
    pub dim: usize,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub min: f32,
    pub max: f32,
    pub seed: i32,
}

impl NoiseDimensions {
    pub fn default(d: usize) -> NoiseDimensions {
        if d < 1 || d > 3 {
            panic!("dimension invalid");
        }
        NoiseDimensions {
            dim: d,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 1,
            height: 1,
            depth: 1,
            min: 0.0,
            max: 1.0,
            seed: 1,
        }
    }

    pub fn len(self) -> usize {
        return self.width * self.width * self.height;
    }
}
