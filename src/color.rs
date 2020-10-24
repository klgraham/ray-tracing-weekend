use rand::prelude::*;
use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Mul, Sub};

// color.rs

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

/// Constant colors
#[derive(Debug, PartialEq)]
pub enum Colors {
    Red,
    Green,
    Blue,
    Black,
    White,
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
}

/// Clamps a color component to [0, 255]
fn clamp_pixel(c: f64) -> u8 {
    if c > 1.0f64 {
        return 255u8;
    } else if c < 0f64 {
        return 0u8;
    } else {
        return (255.0 * c).round() as u8;
    }
}

fn clamp_pixel2(x: f64, x_min: f64, x_max: f64) -> f64 {
    if x < x_min {
        return x_min;
    } else if x > x_max {
        return x_max;
    } else {
        return x;
    }
}

impl Colors {
    pub fn value(self) -> Color {
        match self {
            Colors::Red => Color::new(1.0, 0.0, 0.0),
            Colors::Green => Color::new(0.0, 1.0, 0.0),
            Colors::Blue => Color::new(0.0, 0.0, 1.0),
            Colors::White => Color::new(1.0, 1.0, 1.0),
            Colors::Black => Color::new(0.0, 0.0, 0.0),
        }
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

/// Tests
#[cfg(test)]
mod tests {
    use super::{Color, Colors};
    use crate::color::clamp_pixel;

    //    #[test]
    //    fn can_add_colors() {
    //        // TODO: need to handle roundoff errors for the equality tests
    //        let c1 = Color::new(0.9, 0.6, 0.75);
    //        let c2 = Color::new(0.7, 0.1, 0.25);
    //        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    //    }

    #[test]
    fn can_make_specific_colors() {
        let red = Colors::Red;
        let green = Colors::Green;
        let blue = Colors::Blue;
        let black = Colors::Black;
        let white = Colors::White;
        assert_eq!(red.value(), Color::new(1.0, 0.0, 0.0));
        assert_eq!(green.value(), Color::new(0.0, 1.0, 0.0));
        assert_eq!(blue.value(), Color::new(0.0, 0.0, 1.0));
        assert_eq!(black.value(), Color::new(0.0, 0.0, 0.0));
        assert_eq!(white.value(), Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn can_clamp_pixels() {
        let c = Color::new(1.5, -0.8, 0.5);
        assert_eq!(clamp_pixel(c.red), 255u8);
        assert_eq!(clamp_pixel(c.green), 0u8);
        assert_eq!(clamp_pixel(c.blue), 128u8);
    }
}
