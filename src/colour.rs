use crate::{anyhow, bail, Result};
use std::ops::{Add, Mul};
#[derive(Debug, Copy, Clone, Default, PartialEq)]
/// Represents a RGB colour
pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl Colour {
    /// Create a new Colour object
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Creates a new Colour object from a single hex number i.e. 0xFFFFFF
    pub fn from_hex(hex: u32) -> Result<Self> {
        if hex > 0xFFFFFF {
            bail!("Cannot parse a number bigger than 0xFFFFFF");
        }

        let mut r: u8;
        let mut g: u8;
        let mut b: u8;

        if hex < 0xFFF {
            r = ((hex & 0xF00) >> 8) as u8;
            g = ((hex & 0x0F0) >> 4) as u8;
            b = (hex & 0x00F) as u8;
        } else {
            r = ((hex & 0xFF0000) >> 16) as u8;
            g = ((hex & 0x00FF00) >> 8) as u8;
            b = (hex & 0x0000FF) as u8;
        }

        Ok(Self { r, g, b })
    }

    /// Gets the red component
    pub fn r(&self) -> u8 {
        self.r
    }

    /// Gets the green component
    pub fn g(&self) -> u8 {
        self.g
    }

    /// Gets the blue component
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
