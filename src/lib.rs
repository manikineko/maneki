#![allow(dead_code)]

mod rendering;
mod structure;

#[cfg(test)]
mod test {
    use crate::structure::{gameobj::GameObject, objects::*, texture::Texture};
    use raylib::prelude::*;

    #[test]
    fn window() {
        let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

        rl.hide_cursor();
        let bg_texture = rl.load_texture(&thread, "placeholder.png").unwrap();

        let mut scene = scene::Scene::new("BigBoobs", "yeah");
        let player = player::Player::new("Me!!", "it krista");
        let bg = skybox::Skybox::new(
            "uwu",
            "owo",
            Texture::Image2d(bg_texture, Color::WHITE),
        );

        scene.add(bg);
        scene.add(player);

        while !rl.window_should_close() {
            scene.update(&mut rl, &thread);
        }
    }
}
