use raylib::{color::Color, math::Vector3, prelude::RaylibDraw};

use crate::structure::gameobj::GameObject;

pub struct Skybox {
    name: String,
    uuid: String,
    contents: SkyboxContent,
}

impl Skybox {
    pub fn new(name: String, uuid: String, contents: SkyboxContent) -> Skybox {
        Skybox {
            name,
            uuid,
            contents,
        }
    }
}

pub enum SkyboxContent {
    Color(Color)
}

impl GameObject for Skybox {
    fn update(&mut self, rl: &mut raylib::RaylibHandle, thread: &raylib::RaylibThread) {
        let mut d = rl.begin_drawing(thread);

        match self.contents {
            SkyboxContent::Color(col) => {
                d.clear_background(col)
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
