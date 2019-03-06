use std::collections::HashMap;
use std::sync::RwLock;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

use std::any::{TypeId, Any}; //for a little bit of dynamic typing

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct Entity {
    index: usize,
    generation: usize,
    hash: i128,
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
    entities: RefCell<Vec<Entity>>, //uses RefCell and not RwLock because shouldn't be accessed outside here
    hashes: HashMap<TypeId, i128>,
    hash_base: i128,
    world: HashMap<TypeId, Box<Any>>,
    resources: HashMap<TypeId, Box<Any>>,
}

impl GameState {
    pub fn new() -> GameState {
        let mut w = GameState{entities: RefCell::new(Vec::new()), hashes: HashMap::new(), hash_base: 1, world: HashMap::new(), resources: HashMap::new()};
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
        self.world.entry(TypeId::of::<C>()).or_insert(Box::new(RwLock::new(C::Storage::new())));
        if self.hashes.get(&TypeId::of::<C>()).is_none() {
            //println!("base: {:#0128b}", self.hash_base);
            self.hashes.insert(TypeId::of::<C>(), self.hash_base);
            self.hash_base <<= 1;
        }
    }
    pub fn create_entity(&mut self) -> Entity {
        let e = Entity{index: self.entities.borrow().len(), generation: 0, hash: 0};
        self.entities.borrow_mut().push(e);
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
    //doing this in 'world' bc local copies of Entity might not be correct wrt hashing
    pub fn type_of(&self, entity: Entity) -> i128 {
        self.entities.borrow()[entity.index].hash
    }

    #[allow(unused)]
    fn sweep_delete(&mut self) {
        let mut deleted_entities: Vec<Entity> = Vec::new();
        self.update_all(|e, _: &mut Deleted| {
            deleted_entities.push(e);
        });
        for e in deleted_entities {
            self.entities.borrow_mut()[e.index].generation += 1;
            //TODO: enforce that entities of mismatched generations may not be accessed
        }
        //TODO: reuse free entity slots
        unimplemented!("cannot reuse entities yet");
    }

    //basic crud stuff
    //private bc systems should not operate on individual storages directly    
    fn get_storage<C: Component>(&self) -> &RwLock<C::Storage> {
        self.world[&TypeId::of::<C>()].downcast_ref::<RwLock<C::Storage>>().unwrap()
    }

    pub fn insert<C: Component>(&self, entity: Entity, c: C) {
        self.get_storage::<C>().write().unwrap().insert(entity, c);
        self.entities.borrow_mut()[entity.index].hash ^= self.hashes[&TypeId::of::<C>()];
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
    //(also doesn't lock read for the length of the returned value)
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
        let entities = self.entities.borrow();
        for &e in entities.iter().filter(move |&e| self.is_alive(*e)) {
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

    pub fn read_all<C: Component>(&self, mut f: impl FnMut(Entity, &C)) {
        let lock = self.lock_read::<C>();
        let entities = self.entities.borrow();
        for &e in entities.iter().filter(move |&e| self.is_alive(*e)) {
            if let Some(c) = lock.get(e) {
                f(e, c);
            }
        }
    }
    pub fn read<C: Component>(&self, entity: Entity, mut f: impl FnMut(&C)) {
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
}


//the big one
//able to take a variadic number of &mut Components 
//and run an arbitary function on ALL OF THEM
pub trait SystemRunner<T, F> {
    fn run(&self, f: F);
}

macro_rules! impl_system {
    ($($tp:ident),*) => (
        impl<$($tp),*, Func> SystemRunner<($($tp),*,), Func> for GameState where $($tp: Component),*, Func: FnMut(($(&mut $tp),*,)) {
            #[allow(non_snake_case)] //required until rust has ident_lowercase! or smth
            fn run(&self, mut f: Func) {
                let entities = self.entities.borrow();
                for &i in entities.iter().filter(move |&e| self.is_alive(*e)) {
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
impl_system!(A, B, C, D, E, F);
impl_system!(A, B, C, D, E, F, G);
impl_system!(A, B, C, D, E, F, G, H);

//8 components is plenty for now

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