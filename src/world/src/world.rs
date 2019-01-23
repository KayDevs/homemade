use std::collections::HashMap;
use std::sync::RwLock;
use std::ops::{Deref, DerefMut};

use std::any::{TypeId, Any}; //for a little bit of dynamic typing

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct Entity {
    index: usize,
    generation: usize,
}
impl Entity {
    pub fn id(&self) -> usize {
        self.index
    }
}

pub trait ComponentStorage<C: Component> {
    fn new() -> Self where Self: Sized;
    fn insert(&mut self, entity: Entity, c: C);
    fn delete(&mut self, entity: Entity);
    fn get(&self, entity: Entity) -> Option<&C>;
    fn get_mut(&mut self, entity: Entity) -> Option<&mut C>;
}

pub trait Component: 'static + Sized + Clone {
    type Storage: ComponentStorage<Self>;
}

pub trait Resource: 'static + Sized + Clone {}

use crate::storage::BTreeMapStorage;
#[derive(Clone)]
pub struct Deleted;
impl Component for Deleted {
    type Storage = BTreeMapStorage<Self>;
}

//za warudo
pub struct GameState {
    entities: Vec<Entity>,
    world: HashMap<TypeId, Box<Any>>,
    resources: HashMap<TypeId, Box<Any>>,
}

impl GameState {
    pub fn new() -> GameState {
        let mut w = GameState{entities: Vec::new(), world: HashMap::new(), resources: HashMap::new()};
        w.register_component::<Deleted>();
        w
    }

    pub fn set_resource<R: Resource>(&mut self, resource: R) {
        self.resources.insert(TypeId::of::<R>(), Box::new(resource));
    }
    pub fn get_resource<R: Resource>(&self) -> Option<R> {
        self.resources.get(&TypeId::of::<R>()).map(|r| r.downcast_ref::<R>().unwrap().clone())
    }

    pub fn register_component<C: Component>(&mut self) {
        //wrap up Storage in a RWLock for concurrency :3
        self.world.insert(TypeId::of::<C>(), Box::new(RwLock::new(C::Storage::new())));
    }
    pub fn create_entity(&mut self) -> Entity {
        let e = Entity{index: self.entities.len(), generation: 0};
        self.entities.push(e);
        e
    }
    pub fn delete_entity(&self, entity: Entity) {
        self.insert(entity, Deleted);
    }
    pub fn is_alive(&self, entity: Entity) -> bool {
        self.lock_read::<Deleted>().get(entity).is_none()
    }
    pub fn is_deleted(&self, entity: Entity) -> bool {
        self.lock_read::<Deleted>().get(entity).is_some()
    }

    #[allow(unused)]
    fn sweep_delete(&mut self, entity: Entity) {
        self.entities[entity.id()].generation += 1;
        //todo: make a free list, reuse free entity slots
        //add a 'deleted' flag, or just simply check the free list
        unimplemented!("cannot delete entity {}", entity.id());
    }

    //basic crud stuff
    //private bc systems should not operate on individual storages directly    
    fn get_storage<C: Component>(&self) -> &RwLock<C::Storage> {
        self.world[&TypeId::of::<C>()].downcast_ref::<RwLock<C::Storage>>().unwrap()
    }

    pub fn insert<C: Component>(&self, entity: Entity, c: C) {
        self.get_storage::<C>().write().unwrap().insert(entity, c);
    }
    pub fn delete<C: Component>(&self, entity: Entity) {
        self.lock_write::<C>().delete(entity);
    }
    fn lock_read<C: Component>(&self) -> impl Deref<Target=impl ComponentStorage<C>> + '_ {
        self.get_storage::<C>().read().unwrap()
    }
    fn lock_write<C: Component>(&self) -> impl DerefMut<Target=impl ComponentStorage<C>> + '_ {
        self.get_storage::<C>().write().unwrap()
    }
    
    //returns copies, for simple value reading
    //maybe these functions are a little too misleading...?
    pub fn clone<C: Component>(&self, entity: Entity) -> Option<C> {
        if self.is_alive(entity) {
            self.lock_read::<C>().get(entity).cloned()
        } else {
            None
        }
    }
    //unsafe function
    pub fn get_value<C: Component>(&self, entity: Entity) -> C {
        self.lock_read::<C>().get(entity).unwrap().clone()
    }

    //takes a closure, updates select components
    pub fn update_all<C: Component>(&self, mut f: impl FnMut(Entity, &mut C)) {
        let mut lock = self.lock_write::<C>();
        for &e in self.iter() {
            if let Some(c) = lock.get_mut(e) {
                f(e, c);
            }
        }
    }
    pub fn update<C: Component>(&self, entity: Entity, mut f: impl FnMut(&mut C)) {
        if self.is_alive(entity) {
            if let Some(c) = self.lock_write::<C>().get_mut(entity) {
                f(c);
            }
        }
    }

    pub fn read_all<C: Component>(&self, f: impl Fn(Entity, &C)) {
        let lock = self.lock_read::<C>();
        for &e in self.iter() {
            if let Some(c) = lock.get(e) {
                f(e, c);
            }
        }
    }
    pub fn read<C: Component>(&self, entity: Entity, f: impl Fn(&C)) {
        if self.is_alive(entity) {
            if let Some(c) = self.lock_write::<C>().get(entity) {
                f(c);
            }
        }
    }

    //just a simple check for flag-type components
    pub fn has_flag<C: Component>(&self, entity: Entity) -> bool {
        if self.is_alive(entity) {
            self.lock_read::<C>().get(entity).is_some()
        } else {
            false
        }
    }
    
    //easily iterate only over live entities
    fn iter(&self) -> impl Iterator<Item=&Entity> {
        self.entities.iter().filter(move |&e| self.is_alive(*e))
    }
}


//the big one
//able to take a variadic number of &mut Components and run an arbitary function on ALL OF THEM
pub trait SystemRunner<T, F> {
    fn run(&self, f: F);
}

macro_rules! impl_system {
    ($($tp:ident),*) => (
        impl<$($tp),*, F> SystemRunner<($($tp),*,), F> for GameState where $($tp: Component),*, F: FnMut(($(&mut $tp),*,)) {
            #[allow(non_snake_case)] //required until rust has ident_lowercase! or smth
            fn run(&self, mut f: F) {
                for &i in self.iter() {
                    if let ($(Some($tp)),*,) = ($(self.lock_write::<$tp>().get_mut(i)),*,) {
                        f(($($tp),*,));
                    }
                }
            }
        }
    );
}

impl_system!(A);
impl_system!(A, B);
impl_system!(A, B, C);
impl_system!(A, B, C, D);
impl_system!(A, B, C, D, E);
//5 components is plenty for now

/*impl<T, F> SystemRunner<(T), F> for GameState where T: Component, F: FnMut((&mut T)) {
    fn run(&self, mut f: F) {
        for i in self.iter() {
            if let Some(T) = self.write::<T>().get_mut(i) {
                f(T);
            }
        }
    }
}
impl<T, U, F> SystemRunner<(T, U), F> for GameState where T: Component, U: Component, F: FnMut((&mut T, &mut U)) {
    fn run(&self, mut f: F) {
        for i in self.iter() {
            if let (Some(T), Some(U)) = (self.write::<T>().get_mut(i), self.write::<U>().get_mut(i)) {
                f((T, U));
            }
        }
    }
}
impl<T, U, V, F> SystemRunner<(T, U, V), F> for GameState where T: Component, U: Component, V: Component, F: FnMut((&mut T, &mut U, &mut V)) {
    fn run(&self, mut f: F) {
        for i in self.iter() {
            if let (Some(T), Some(U), Some(V)) = (self.write::<T>().get_mut(i), self.write::<U>().get_mut(i), self.write::<V>().get_mut(i)) {
                f((T, U, V));
            }
        }
    }
}*/