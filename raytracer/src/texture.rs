use crate::color::Color;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub enum Texture {
    /// Single flat color everywhere.
    Solid(Color),

    /// 3D checkerboard: alternates between two colors based on which unit
    /// cell of world space the hit point falls in. Works on any object
    /// since it only looks at the hit point, not surface UVs.
    Checker {
        a: Color,
        b: Color,
        scale: f64, // size of each square; smaller = smaller squares
    },

    /// Stripes along the Y axis (e.g. for a cylinder or cube "barber pole").
    Stripes {
        a: Color,
        b: Color,
        scale: f64,
    },

    /// Latitude/longitude bands on a sphere, based on the surface normal
    /// rather than world position (so it doesn't stretch with radius/center).
    SphereGrid {
        a: Color,
        b: Color,
        lat_bands: f64,
        lon_bands: f64,
    },
}

impl Texture {
    pub fn color_at(&self, point: Vec3, normal: Vec3) -> Color {
        match self {
            Texture::Solid(color) => *color,

            Texture::Checker { a, b, scale } => {
                let s = (point.x / scale).floor()
                    + (point.y / scale).floor()
                    + (point.z / scale).floor();
                if (s as i64) % 2 == 0 {
                    *a
                } else {
                    *b
                }
            }

            Texture::Stripes { a, b, scale } => {
                let s = (point.y / scale).floor() as i64;
                if s.rem_euclid(2) == 0 {
                    *a
                } else {
                    *b
                }
            }

            Texture::SphereGrid {
                a,
                b,
                lat_bands,
                lon_bands,
            } => {
                let n = normal.normalize();
                let lat = n.y.clamp(-1.0, 1.0).acos(); // 0..PI
                let lon = n.z.atan2(n.x); // -PI..PI

                let lat_idx = (lat / std::f64::consts::PI * lat_bands).floor() as i64;
                let lon_idx =
                    ((lon + std::f64::consts::PI) / (2.0 * std::f64::consts::PI) * lon_bands)
                        .floor() as i64;

                if (lat_idx + lon_idx).rem_euclid(2) == 0 {
                    *a
                } else {
                    *b
                }
            }
        }
    }
}
