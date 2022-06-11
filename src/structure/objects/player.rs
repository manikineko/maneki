use raylib::consts::MouseButton;
use raylib::{color::Color, math::Vector3, prelude::RaylibDraw, RaylibThread};
use raylib::consts::KeyboardKey;
use crate::structure::controller::Controller;
use crate::structure::gameobj::GameObject;

#[derive(Debug, Clone, Default)]
/// Ingame player type
pub struct Player {
    controller: Controller,
    local: bool,
    position: Vector3,
    uuid: String,
    name: String,
    color: Color,
    //model: Model,
}

impl Player {
    pub fn new<S: Into<String>>(name: S, uuid: S) -> Player {
        Self {
            uuid: uuid.into(),
            name: name.into(),
            ..Default::default()
        }
    }
}

impl GameObject for Player {
    fn update(&mut self, rh: &mut raylib::RaylibHandle, _: &RaylibThread) {
        if rh.is_key_down(KeyboardKey::KEY_W) {
            
            self.position.y -= 0.1;
        }
        if rh.is_key_down(KeyboardKey::KEY_S) {
            
            self.position.y += 0.1;
        }
        if rh.is_key_down(KeyboardKey::KEY_A) {
            self.position.x -= 0.1;
        }
        if rh.is_key_down(KeyboardKey::KEY_D) {
            self.position.x += 0.1;
        }

        

        if rh.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            self.color = Color::PINK;
        } else {
            self.color = Color::BLACK;
        }
    }

    fn draw(&mut self, d: &mut raylib::prelude::RaylibDrawHandle, _: &RaylibThread) {
        d.draw_circle(
            self.position.x as i32,
            self.position.y as i32,
            10.0,
            self.color,
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
