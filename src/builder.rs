use crate::{anyhow, bail, Result};
use crate::{Colour, Gradient, StyleOptions};
use std::default::Default;

/// Builder type for `Gradient`
pub struct GradientBuilder {
    is_bold: bool,
    is_italic: bool,
    start_colour: Colour,
    end_colour: Colour,
    text: String,
}

impl GradientBuilder {
    /// Create a new gradient builder object
    pub fn new() -> Self {
        Self {
            is_italic: false,
            is_bold: false,
            start_colour: Colour::default(),
            end_colour: Colour::default(),
            text: String::default(),
        }
    }

    /// Set the text to be displayed as bold
    pub fn bold(mut self) -> Self {
        self.is_bold = true;
        self
    }

    /// Set the text to be displayed in italics
    pub fn italic(mut self) -> Self {
        self.is_italic = true;
        self
    }

    /// Remove all style options from the text
    pub fn plain(mut self) -> Self {
        self.is_bold = false;
        self.is_italic = false;
        self
    }

    /// Set the text rendered by the gradient
    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = text.into();
        self
    }

    /// Set the start colour
    pub fn start_colour(mut self, colour: Colour) -> Self {
        self.start_colour = colour;
        self
    }

    /// Set the end colour
    pub fn end_colour(mut self, colour: Colour) -> Self {
        self.end_colour = colour;
        self
    }

    /// Create the final Gradient type
    pub fn build(self) -> Result<Gradient> {
        if self.start_colour == Colour::default() || self.end_colour == Colour::default() {
            return Err(anyhow!("Cannot build a gradient with unassigned colours"));
        }

        Ok(Gradient {
            text: self.text,
            start: self.start_colour,
            end: self.end_colour,
            options: match (self.is_bold, self.is_italic) {
                (true, true) => StyleOptions::BoldItalic,
                (false, true) => StyleOptions::Italic,
                (true, false) => StyleOptions::Bold,
                (false, false) => StyleOptions::Default,
            },
        })
    }
}

impl Default for GradientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
