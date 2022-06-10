use raylib::{math::Vector3, RaylibThread};

use crate::structure::gameobj::GameObject;

#[derive(Default)]
pub struct Scene {
    objects: Vec<Box<dyn GameObject>>,
    name: String,
    uuid: String,
}

impl Scene {
    pub fn new<S: Into<String>>(name: S, uuid: S) -> Scene {
        Self {
            uuid: uuid.into(),
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn add<G: GameObject + 'static>(&mut self, obj: G) {
        self.objects.push(GameObject::boxed(obj))
    }
}

impl GameObject for Scene {
    fn update(&mut self, rl: &mut raylib::RaylibHandle, thread: &RaylibThread) {
        self.objects
            .iter_mut()
            .for_each(|obj| obj.update(rl, thread));

        let mut d = rl.begin_drawing(thread);
        self.draw(&mut d, thread);
    }

    fn draw(&mut self, d: &mut raylib::prelude::RaylibDrawHandle, thread: &RaylibThread) {
        self.objects
            .iter_mut()
            .for_each(|obj| obj.draw(d, thread));
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

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_uuid(&mut self, uuid: String) {
        self.uuid = uuid;
    }
}
