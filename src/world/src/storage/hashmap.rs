use std::collections::HashMap;
use crate::world::{Component, ComponentStorage};

pub type HashMapStorage<C> = HashMap<usize, C>;

impl<C> ComponentStorage<C> for HashMap<usize, C> where C: Component + 'static + Sized {
    fn new() -> HashMap<usize, C> {
        HashMap::new()
    }
    fn insert(&mut self, entity: usize, c: C) {
        self.insert(entity, c);
    }
    fn delete(&mut self, entity: usize) {
        self.remove(&entity);
    }
    fn get(&self, entity: usize) -> Option<&C> {
        self.get(&entity)
    }
    fn get_mut(&mut self, entity: usize) -> Option<&mut C> {
        self.get_mut(&entity)
    }
}