use rand::prelude::*;
use std::cmp::PartialEq;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul, Sub};

// color.rs

/// RGB color
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue }
    }

    pub fn mult(&self, other: Color) -> Color {
        Color::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }

    pub fn to_pixel(&self) -> (u8, u8, u8) {
        (
            clamp_pixel(self.red),
            clamp_pixel(self.green),
            clamp_pixel(self.blue),
        )
    }

    pub fn sample_pixel(&self, samples_per_pixel: u32) -> (u8, u8, u8) {
        // divide the color by the number of samples
        let scale = 1.0 / (samples_per_pixel as f64);
        let r = self.red * scale;
        let g = self.green * scale;
        let b = self.blue * scale;

        (
            clamp_pixel(r.sqrt()),
            clamp_pixel(g.sqrt()),
            clamp_pixel(b.sqrt()),
        )
    }

    pub fn random() -> Color {
        let mut rng = rand::thread_rng();
        Color::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub const RED: Self = Self {
        red: 1.0,
        green: 0.0,
        blue: 0.0,
    };
    pub const GREEN: Self = Self {
        red: 0.0,
        green: 1.0,
        blue: 0.0,
    };
    pub const BLUE: Self = Self {
        red: 0.0,
        green: 0.0,
        blue: 1.0,
    };
    pub const BLACK: Self = Self {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };
    pub const WHITE: Self = Self {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };
    pub const CINNABAR: Self = Self {
        red: 0.73,
        green: 0.27,
        blue: 0.21,
    };
    pub const DIAMOND: Self = Self {
        red: 0.78,
        green: 0.88,
        blue: 0.91,
    };

    pub fn diffuse_albedo() -> Self {
        Color::random() * Color::random()
    }

    pub fn metal_albedo() -> Self {
        Color::random()
    }
}

/// Clamps a color component to [0, 255]
fn clamp_pixel(c: f64) -> u8 {
    match c {
        c if c > 1.0f64 => 255u8,
        c if c < 0f64 => 0u8,
        _ => (255.0 * c).round() as u8,
    }
}

fn clamp_pixel2(x: f64, x_min: f64, x_max: f64) -> f64 {
    match x {
        x if x < x_min => x_min,
        x if x > x_max => x_max,
        _ => x,
    }
}

/// Color addition
impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }
}

impl<'a> Add<&'a Color> for Color {
    type Output = Color;

    fn add(self, other: &Color) -> Color {
        Color::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }
}

/// Color addition, with assignment
impl AddAssign<Color> for Color {
    fn add_assign(&mut self, other: Color) {
        self.red += other.red;
        self.green += other.green;
        self.blue += other.blue;
    }
}

/// Color subtraction
impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color::new(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue,
        )
    }
}

/// Scalar multiplication for Color
impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, a: f64) -> Color {
        Color::new(self.red * a, self.green * a, self.blue * a)
    }
}

/// Scalar multiplication for Color
impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color::new(other.red * self, other.green * self, other.blue * self)
    }
}

/// Multiplication (Elementwise) for Color (Hadamard product)
impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }
}

impl<'a> Sum<&'a Color> for Color {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Color::BLACK, Color::add)
    }
}

/// Tests
#[cfg(test)]
mod tests {
    use super::Color;
    use crate::color::clamp_pixel;

    //    #[test]
    //    fn can_add_colors() {
    //        // TODO: need to handle roundoff errors for the equality tests
    //        let c1 = Color::new(0.9, 0.6, 0.75);
    //        let c2 = Color::new(0.7, 0.1, 0.25);
    //        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    //    }

    #[test]
    fn can_clamp_pixels() {
        let c = Color::new(1.5, -0.8, 0.5);
        assert_eq!(clamp_pixel(c.red), 255u8);
        assert_eq!(clamp_pixel(c.green), 0u8);
        assert_eq!(clamp_pixel(c.blue), 128u8);
    }
}
