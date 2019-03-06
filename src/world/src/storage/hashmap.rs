use std::collections::HashMap;
use crate::world::{Entity, Component, ComponentStorage};

pub type HashMapStorage<C> = HashMap<usize, C>;

impl<C> ComponentStorage<C> for HashMap<usize, C> where C: Component + 'static + Sized {
    fn new() -> HashMap<usize, C> {
        HashMap::new()
    }
    fn insert(&mut self, entity: Entity, c: C) {
        self.insert(entity.id(), c);
    }
    fn delete(&mut self, entity: Entity) {
        self.remove(&entity.id());
    }
    fn get(&self, entity: Entity) -> Option<&C> {
        self.get(&entity.id())
    }
    fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        self.get_mut(&entity.id())
    }
}