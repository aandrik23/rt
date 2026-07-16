use crate::color::Color;
use crate::texture::Texture;

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub texture: Texture, // surface color/pattern, e.g. Texture::Solid(Color::new(0.7, 0.8, 0.3))
    pub shininess: f64,   // Phong exponent, e.g. 32.0-256.0
    pub ambient: f64,     // ambient coefficient, e.g. 0.1
}

impl Material {
    /// Convenience constructor for a flat, untextured material (old `albedo` behavior).
    pub fn solid(color: Color, shininess: f64, ambient: f64) -> Self {
        Material {
            texture: Texture::Solid(color),
            shininess,
            ambient,
        }
    }
}
