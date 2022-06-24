use raylib::{color::Color, texture::Texture2D};


/// Use as literal
#[derive(Debug)]
pub enum Texture {
    Color(Color),

    /// Fallback clear colour
    Image2d(Texture2D, Color)
}