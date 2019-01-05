use std::collections::HashMap;
use crate::world::{Entity, Component, ComponentStorage};

pub type HashMapStorage<C> = HashMap<Entity, C>;

impl<C> ComponentStorage<C> for HashMap<Entity, C> where C: Component + 'static + Sized {
    fn new() -> HashMap<Entity, C> {
        HashMap::new()
    }
    fn insert(&mut self, entity: Entity, c: C) {
        self.insert(entity, c);
    }
    fn delete(&mut self, entity: Entity) {
        self.remove(&entity);
    }
    fn get(&self, entity: Entity) -> Option<&C> {
        self.get(&entity)
    }
    fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        self.get_mut(&entity)
    }
}