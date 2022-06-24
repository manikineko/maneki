use crate::structure::controller::Controller;
use crate::structure::gameobj::GameObject;
use maneki_macros::game_object;
use raylib::consts::KeyboardKey;
use raylib::consts::MouseButton;
use raylib::{color::Color, prelude::RaylibDraw, RaylibThread};

#[game_object(
    update = update,
    draw = draw,
    props = {
        controller: Controller,
        local: bool,
        color: Color 
    }
)]
/// Ingame player type
pub struct Player;

fn update(instance: &mut Player, rh: &mut raylib::RaylibHandle, _: &RaylibThread) {
    if rh.is_key_down(KeyboardKey::KEY_W) {
        instance.position.y -= 0.1;
    }
    if rh.is_key_down(KeyboardKey::KEY_S) {
        instance.position.y += 0.1;
    }
    if rh.is_key_down(KeyboardKey::KEY_A) {
        instance.position.x -= 0.1;
    }
    if rh.is_key_down(KeyboardKey::KEY_D) {
        instance.position.x += 0.1;
    }

    if rh.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
        instance.color = Color::PINK;
    } else {
        instance.color = Color::BLACK;
    }
}

fn draw(instance: &mut Player, d: &mut raylib::prelude::RaylibDrawHandle, _: &RaylibThread) {
    d.draw_circle(
        instance.position.x as i32,
        instance.position.y as i32,
        20.0,
        instance.color,
    )
}
