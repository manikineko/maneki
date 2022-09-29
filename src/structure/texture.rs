use raylib::{color::Color, texture::WeakTexture2D};

/// Use as literal
#[derive(Debug, Clone)]
pub enum Texture {
    Color(Color),

    Image2d(WeakTexture2D, Color),
}
