use raylib::{color::Color, math::Vector3, prelude::RaylibDraw, RaylibThread};

use crate::structure::gameobj::GameObject;
use crate::structure::controller::Controller;
#[derive(Debug, Clone, Default)]
/// Ingame player type
pub struct Player {
    controller: Controller,
    local: bool,
    position: Vector3,
    uuid: String,
    name: String,
    //model: Model,
}

impl Player {
    pub fn new() -> Player {
        Default::default()
    }
}

impl GameObject for Player {
    fn update(&mut self, rh: &mut raylib::RaylibHandle, thread: &RaylibThread) {
        self.position.y += 0.01;
        
        if self.position.y as i32 >= rh.get_screen_height() - 10 {
            self.position.y = 10.0;
        }

        let mut d = rh.begin_drawing(thread);
        self.draw(&mut d, thread)
    }

    fn draw(&mut self, d: &mut raylib::prelude::RaylibDrawHandle, _: &RaylibThread) {
        d.draw_circle(
            self.position.x as i32,
            self.position.y as i32,
            10.0,
            Color::BLACK,
        )
    }

    fn position(&self) -> raylib::math::Vector3 {
        self.position
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn uuid(&self) -> String {
        self.uuid.clone()
    }

    fn set_position(&mut self, pos: raylib::math::Vector3) {
        self.position = pos;
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_uuid(&mut self, uuid: String) {
        self.uuid = uuid;
    }
}
