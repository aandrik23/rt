use crate::color::Color;
use crate::vec3::Vec3;

pub struct PointLight {
    pub position: Vec3,
    pub color: Color,   // e.g. Color(1.0, 1.0, 1.0) for white
    pub intensity: f64, // brightness multiplier, e.g. 1.0
}
