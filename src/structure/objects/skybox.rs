use raylib::{color::Color, math::Vector3, prelude::RaylibDraw};

use crate::structure::{gameobj::GameObject, texture::Texture};

pub struct Skybox {
    name: String,
    uuid: String,
    texture: Texture,
}

impl Skybox {
    pub fn new<S: Into<String>>(name: S, uuid: S, texture: Texture) -> Skybox {
        Skybox {
            name: name.into(),
            uuid: uuid.into(),
            texture,
        }
    }
}

impl GameObject for Skybox {
    fn draw(&mut self, d: &mut raylib::prelude::RaylibDrawHandle, _: &raylib::RaylibThread) {
        match &self.texture {
            Texture::Color(col) => d.clear_background(col),
            Texture::Image2d(texture, clear) => {
                d.clear_background(clear);
                let w = d.get_screen_width() as f32;
                let h = d.get_screen_height() as f32;

                d.draw_texture(texture, w as i32, h as i32, Color::BLACK)
            },
        }
    }

    fn position(&self) -> raylib::math::Vector3 {
        Vector3::zero()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn uuid(&self) -> String {
        self.uuid.clone()
    }
}
