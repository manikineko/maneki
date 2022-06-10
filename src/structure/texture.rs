use raylib::{color::Color, texture::Texture2D};


/// Use as literal
pub enum Texture {
    Color(Color),

    /// Fallback clear colour
    Image2d(Texture2D, Color)
}