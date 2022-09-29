use self::gameobj::GameObject;

pub type Children = Vec<Box<dyn GameObject>>;

pub mod gameobj;
pub mod controller;
pub mod objects;
pub mod texture;