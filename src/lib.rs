use colored::Colorize;
use std::fmt::{self, Display, Formatter};

mod colour;
pub use colour::Colour;

mod builder;
pub use builder::GradientBuilder;

pub use anyhow::{anyhow, bail, Result};


#[derive(Debug)]
enum StyleOptions {
    Bold,
    Italic,
    BoldItalic,
    Default,
}

#[derive(Debug)]
/// Representation of text with a gradient applied to it
pub struct Gradient {
    start: Colour,
    end: Colour,
    text: String,
    options: StyleOptions,
}

impl Gradient {
    /// Generate a new gradient
    pub fn new(start: Colour, end: Colour, text: &str) -> Self {
        Self {
            start,
            end,
            text: text.to_owned(),
            options: StyleOptions::Default,
        }
    }

    fn get_gradient(&self) -> Vec<Colour> {
        let mut colours = Vec::new();
        let text_length = self.text.len();

        for (idx, _) in self.text.chars().enumerate() {
            let alpha: f32 = idx as f32 / text_length as f32;
            let beta: f32 = 1.0 - alpha;
            colours.push(self.start * alpha + self.end * beta);
        }

        colours
    }

    fn format(&self) -> String {
        use crate::StyleOptions::*;
        self.text
            .chars()
            .zip(self.get_gradient().iter())
            .map(|(character, colour)| {
                let s = String::from(character).truecolor(colour.r(), colour.g(), colour.b());
                format!(
                    "{}",
                    match self.options {
                        Default => s,
                        Bold => s.bold(),
                        Italic => s.italic(),
                        BoldItalic => s.bold().italic(),
                    }
                )
            })
            .collect()
    }
}

impl Display for Gradient {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient() -> Result<()> {
        let g = GradientBuilder::new()
            .text("Hello, World!".to_string())
            .start_colour(Colour::from_hex(0x24f26f)?)
            .end_colour(Colour::from_hex(0x8424f2)?)
            .build()?;
        let s = format!("{g}");

        assert_eq!(s.len(), 299);
        Ok(())
    }
}
