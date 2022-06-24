use maneki_macros::game_object;
use raylib::{color::Color, prelude::RaylibDraw};

use crate::structure::{gameobj::GameObject, texture::Texture};

#[game_object(
    draw = draw,
    props = {
        texture: Texture,
    }
)]
pub struct Skybox;

fn draw(instance: &mut Skybox, d: &mut raylib::prelude::RaylibDrawHandle, _: &raylib::RaylibThread) {
    match &instance.texture {
        Texture::Color(col) => d.clear_background(col),
        Texture::Image2d(texture, clear) => {
            d.clear_background(clear);
            let w = d.get_screen_width() as f32;
            let h = d.get_screen_height() as f32;

            d.draw_texture(texture, w as i32, h as i32, Color::BLACK)
        },
    }
}
