use raylib::{color::Color, math::Vector3, prelude::RaylibDraw};

use crate::structure::gameobj::GameObject;

pub struct Skybox {
    name: String,
    uuid: String,
    contents: SkyboxContent,
}

impl Skybox {
    pub fn new<S: Into<String>>(name: S, uuid: S, contents: SkyboxContent) -> Skybox {
        Skybox {
            name: name.into(),
            uuid: uuid.into(),
            contents,
        }
    }
}

pub enum SkyboxContent {
    Color(Color),
}

impl GameObject for Skybox {
    fn draw(&mut self, d: &mut raylib::prelude::RaylibDrawHandle, _: &raylib::RaylibThread) {
        match self.contents {
            SkyboxContent::Color(col) => d.clear_background(col),
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
