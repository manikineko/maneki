use std::fmt::Debug;
use raylib::{math::Vector3, prelude::RaylibDrawHandle, RaylibHandle, RaylibThread};


#[allow(unused_variables)]
pub trait GameObject
where
    Self: 'static + Send + Sync + Debug,
{
    // Lifecycle
    fn init(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {}
    fn update(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {}
    fn draw(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread) {}

    // get/set
    fn position(&self) -> Vector3;
    fn name(&self) -> String;
    fn uuid(&self) -> String;

    fn set_position(&mut self, pos: Vector3) {}
    fn set_name(&mut self, name: String) {}
    fn set_uuid(&mut self, uuid: String) {}

    fn boxed(&self) -> Box<dyn GameObject>;
}

impl Clone for Box<dyn GameObject> {
    fn clone(&self) -> Self {
        self.boxed()
    }
}
