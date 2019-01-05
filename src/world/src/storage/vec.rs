use crate::world::{Entity, Component, ComponentStorage};

pub type VecStorage<C> = Vec<Option<C>>;

impl<C> ComponentStorage<C> for Vec<Option<C>> where C: Component + 'static + Sized {
    fn new() -> Vec<Option<C>> {
        Vec::new()
    }
    fn insert(&mut self, entity: Entity, c: C) {
        if entity.id() >= self.len() {
            self.reserve(entity.id() - self.len() + 1);
            for _ in 0..=entity.id() - self.len() {
                self.push(None);
            }
        }
        self[entity.id()] = Some(c);
    }
    fn delete(&mut self, entity: Entity) {
        if entity.id() < self.len() {
            self[entity.id()] = None;
        }
    }
    fn get(&self, entity: Entity) -> Option<&C> {
        if entity.id() >= self.len() {
            None
        } else {
            self[entity.id()].as_ref()
        }
    }
    fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        if entity.id() >= self.len() {
            None
        } else {
            self[entity.id()].as_mut()
        }
    }
}

