use std::collections::BTreeMap;
use crate::world::{Component, ComponentStorage};

pub type BTreeMapStorage<C> = BTreeMap<usize, C>;

impl<C> ComponentStorage<C> for BTreeMap<usize, C> where C: Component + 'static + Sized {
    fn new() -> BTreeMap<usize, C> {
        BTreeMap::new()
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