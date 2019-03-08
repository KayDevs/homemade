use crate::world::{Component, ComponentStorage};

pub type VecStorage<C> = Vec<Option<C>>;

impl<C> ComponentStorage<C> for Vec<Option<C>> where C: Component + 'static + Sized {
    fn new() -> Vec<Option<C>> {
        Vec::new()
    }
    fn insert(&mut self, entity: usize, c: C) {
        if entity >= self.len() {
            self.reserve(entity - self.len() + 1);
            for _ in 0..=entity - self.len() {
                self.push(None);
            }
        }
        self[entity] = Some(c);
    }
    fn delete(&mut self, entity: usize) {
        if entity < self.len() {
            self[entity] = None;
        }
    }
    fn get(&self, entity: usize) -> Option<&C> {
        if entity >= self.len() {
            None
        } else {
            self[entity].as_ref()
        }
    }
    fn get_mut(&mut self, entity: usize) -> Option<&mut C> {
        if entity >= self.len() {
            None
        } else {
            self[entity].as_mut()
        }
    }
}

