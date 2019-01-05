use std::collections::BTreeMap;
use crate::world::{Entity, Component, ComponentStorage};

pub type BTreeMapStorage<C> = BTreeMap<Entity, C>;

impl<C> ComponentStorage<C> for BTreeMap<Entity, C> where C: Component + 'static + Sized {
    fn new() -> BTreeMap<Entity, C> {
        BTreeMap::new()
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