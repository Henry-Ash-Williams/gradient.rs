use std::ops::{Add, Mul};
#[derive(Debug, Copy, Clone, Default)]
pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }
}

impl Add<Colour> for Colour {
    type Output = Colour;

    fn add(self, other: Self) -> Self::Output {
        Colour {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Mul<f32> for Colour {
    type Output = Colour;

    fn mul(self, scalar: f32) -> Self::Output {
        Colour {
            r: (self.r as f32 * scalar) as u8,
            g: (self.g as f32 * scalar) as u8,
            b: (self.b as f32 * scalar) as u8,
        }
    }
}
