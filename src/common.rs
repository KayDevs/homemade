//some common components (VecStorage-level common)

extern crate world;
use world::Component;
use world::storage::VecStorage;

#[derive(Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}
impl Component for Position {
    type Storage = VecStorage<Self>;
}

//for indexing
#[derive(Clone)]
pub struct Name(pub &'static str);
impl Component for Name {
    type Storage = VecStorage<Self>;
}
//for rendering
#[derive(Clone)]
pub struct RenderInfo(pub &'static str);
impl Component for RenderInfo {
    type Storage = VecStorage<Self>;
}

