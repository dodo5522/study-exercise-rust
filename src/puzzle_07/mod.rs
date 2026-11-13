use std::f32::consts::PI;

pub struct Degrees(pub f32);
pub struct Radians(pub f32);

impl Degrees {
    pub fn new(angle: f32) -> Self {
        Self(angle)
    }
}

impl From<Degrees> for Radians {
    fn from(degrees: Degrees) -> Self {
        Self(degrees.0 * PI / 180.0)
    }
}

impl From<Radians> for Degrees {
    fn from(radians: Radians) -> Self {
        Self(radians.0 * 180.0 / PI)
    }
}
