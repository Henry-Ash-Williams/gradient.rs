use crate::{Colour, Gradient, StyleOptions};
use std::default::Default;

pub struct GradientBuilder {
    is_bold: bool,
    is_italic: bool,
    start_colour: Colour,
    end_colour: Colour,
    text: String,
}

impl GradientBuilder {
    pub fn new() -> Self {
        Self {
            is_italic: false,
            is_bold: false,
            start_colour: Colour::default(),
            end_colour: Colour::default(),
            text: String::default(),
        }
    }

    pub fn bold(mut self) -> Self {
        self.is_bold = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.is_italic = true;
        self
    }

    pub fn plain(mut self) -> Self {
        self.is_bold = false;
        self.is_italic = false;
        self
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn start_colour(mut self, colour: Colour) -> Self {
        self.start_colour = colour;
        self
    }

    pub fn end_colour(mut self, colour: Colour) -> Self {
        self.end_colour = colour;
        self
    }

    pub fn build(self) -> Gradient {
        Gradient {
            text: self.text,
            start: self.start_colour,
            end: self.end_colour,
            options: if self.is_bold && self.is_italic {
                StyleOptions::BoldItalic
            } else if self.is_bold && !self.is_italic {
                StyleOptions::Bold
            } else if !self.is_bold && self.is_italic {
                StyleOptions::Italic
            } else {
                StyleOptions::Default
            },
        }
    }
}

impl Default for GradientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
