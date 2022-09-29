use maneki_macros::game_object;
use raylib::RaylibThread;

use crate::structure::gameobj::GameObject;

#[game_object(
    update = update,
    draw = draw,
)]
pub struct Scene;

fn update(instance: &mut Scene, rl: &mut raylib::RaylibHandle, thread: &RaylibThread) {
    instance
        .children
        .iter_mut()
        .for_each(|obj| obj.update(rl, thread));
    
    let mut d = rl.begin_drawing(thread);
    instance.draw(&mut d, thread);
}

fn draw(instance: &mut Scene, d: &mut raylib::prelude::RaylibDrawHandle, thread: &RaylibThread) {
    instance
        .children
        .iter_mut()
        .for_each(|obj| obj.draw(d, thread));
}
