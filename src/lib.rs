#![allow(dead_code)]

mod rendering;
mod structure;

#[cfg(test)]
mod test {
    use crate::structure::{gameobj::GameObject, objects::*};
    use raylib::prelude::*;

    #[test]
    fn window() {
        let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

        let mut scene = scene::Scene::new();
        let player = player::Player::new();
        let bg = skybox::Skybox::new(
            "uwu".to_string(),
            "owo".to_string(),
            skybox::SkyboxContent::Color(Color::WHITE),
        );

        scene.add(player);
        scene.add(bg);

        while !rl.window_should_close() {
            scene.update(&mut rl, &thread);
        }
    }
}
