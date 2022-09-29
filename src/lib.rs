#![allow(dead_code)]

mod rendering;
mod steam;
mod structure;

#[cfg(test)]
#[allow(unused)]
mod test {
    use crate::steam::steam_init;
    use crate::structure::{
        controller::Controller, gameobj::GameObject, objects::*, texture::Texture,
    };
    use raylib::prelude::*;

    fn update(s: &mut Test, rl: &mut RaylibHandle, thread: &RaylibThread) {}
    fn init(s: &mut Test, rl: &mut RaylibHandle, thread: &RaylibThread) {}
    fn draw(s: &mut Test, rl: &mut RaylibDrawHandle, thread: &RaylibThread) {}

    #[maneki_macros::game_object(
        update = |a, b, c| update(a, b, c),
        draw = draw,
        init = init,
        props = {
            hug: bool,
            uwu: bool
        }
    )]
    struct Test;

    #[test]
    fn window() {
        let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

        rl.hide_cursor();
        // safety: uhhhhh
        let bg_texture = unsafe {rl.load_texture(&thread, "placeholder.png").unwrap().make_weak()};

        let mut scene = scene::Scene::new("BigBoobs", "yeah");
        let player = player::Player::new("Me!!", "it krista", Controller(0), true, Color::BLACK);
        let bg = skybox::Skybox::new("uwu", "owo", Texture::Image2d(bg_texture, Color::WHITE));

        scene.add(bg);
        scene.add(player);
        let (steam_client, steam_single) = steam_init();
        while !rl.window_should_close() {
            
            scene.update(&mut rl, &thread);
        }
    }

    #[test]
    fn uwu() {
        let a = Test::new("Test", "Test", true, true);
        dbg!(a);
    }
}
