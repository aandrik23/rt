use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn black() -> Self {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn clamp(&self, min: f64, max: f64) -> Color {
        Color::new(
            self.r.clamp(min, max),
            self.g.clamp(min, max),
            self.b.clamp(min, max),
        )
    }

    // Gamma 2 correction (sqrt each channel)
    pub fn gamma_correct(&self) -> Color {
        Color::new(
            self.r.max(0.0).sqrt(),
            self.g.max(0.0).sqrt(),
            self.b.max(0.0).sqrt(),
        )
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

// Component-wise multiply (e.g. albedo * light color)
impl Mul for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, scalar: f64) -> Color {
        Color::new(self.r * scalar, self.g * scalar, self.b * scalar)
    }
}
