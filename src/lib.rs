#![allow(dead_code)]

mod rendering;
mod structure;
mod steam;
#[cfg(test)]
#[allow(unused)]
mod test {
    use crate::{structure::{gameobj::GameObject, objects::*, texture::Texture, controller::Controller}, rendering::model::FBXFile};
    use raylib::prelude::*;
    use crate::steam::steam_init;

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
        let bg_texture = rl.load_texture(&thread, "bg.png").unwrap();

        let mut scene = scene::Scene::new("BigBoobs", "yeah", Vec::new());
        let player = player::Player::new("Me!!", "it krista", Controller(0), true, Color::BLACK);
        let bg = skybox::Skybox::new(
            "uwu",
            "owo",
            Texture::Image2d(bg_texture, Color::WHITE),
        );
        
        scene.add(bg);
        scene.add(player);
        let (steam_client,steam_single) = steam_init();
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
