use world::{GameState, Entity, Component};
use world::storage::{VecStorage, HashMapStorage};
use crate::stats;
use crate::stats::Stat;
use crate::common::{Name, Position};

#[derive(Clone)]
pub struct Inventory {
    pub items: Vec<Entity>,
}
impl Component for Inventory {
    type Storage = HashMapStorage<Self>;
}
impl Inventory {
    pub fn new() -> Inventory {
        Inventory{items: Vec::new()}
    }
}

//3 things items can be, specifically, so far :3
//right-click menu: Use, Equip
#[derive(Clone)]
pub struct Consumable {
    buffs: Vec<(Stat, i32)>,
}
impl Component for Consumable {
    type Storage = VecStorage<Self>;
}
impl Consumable {
    pub fn new(buffs: Vec<(Stat, i32)>) -> Consumable {
        Consumable{buffs}
    }
}

/*#[derive(Clone)]
//where T is clothing/armour/weapons/enchantment/whatever
pub struct Equipment<T> {
    slots: Vec<Option<Index>>, //invariant: should be len() = slots
    _marker: std::marker::PhantomData<T>,
}
impl<T> Equipment<T> where T: Clone + 'static {
    pub fn new(num_slots: usize) -> Equipment<T> {
        let mut e = Equipment{slots: Vec::with_capacity(num_slots), _marker: std::marker::PhantomData};
        for _ in 0..num_slots {
            e.slots.push(None);
        }
        e
    }
    pub fn add_slots(&mut self, num_slots: usize) {
        self.slots.reserve(num_slots);
        for _ in 0..num_slots {
            self.slots.push(None);
        }
    }
    pub fn equipment(&self) -> &Vec<Option<Index>> {
        &self.slots
    }
    pub fn insert(&mut self, item: Index) -> Option<usize> {
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(item);
                return Some(i);
            }
        }
        println!("No more slots.");
        return None;
    }
    pub fn remove(&mut self, item: Index) {
        if self.slots.contains(&Some(item)) {
            for i in 0..self.slots.len() {
                if self.slots[i] == Some(item) {
                    self.slots[i] = None;
                }
            }
        }
    }
}
impl<T> Component for Equipment<T> where T: Clone + 'static {
    type Storage = HashMapStorage<Self>;
}

#[derive(Clone)]
pub struct Equippable<T> {
    item_slot: Option<usize>, //position in equipment list, None if unequipped
    _marker: std::marker::PhantomData<T>,
}
impl<T> Component for Equippable<T> where T: Clone + 'static {
    type Storage = VecStorage<Self>;
}

impl<T> Equippable<T> where T: Component {
    pub fn new() -> Equippable<T> {
        Equippable{item_slot: None, _marker: std::marker::PhantomData}
    }
}
/*pub fn equip<T: Component>(w: &GameState, equipper: Index, item: Index) {
    w.update(equipper, |eqp: &mut Equipment<T>| {
        w.update(item, |_: &mut T| {
            w.insert(item, eqp.insert(item))
            
        });
    });
}
pub fn unequip<T: Component>(w: &GameState, equipper: Index, item: Index) {
    w.update(item, |e: &mut Equippable<T>| {
        if e.item_slot.is_some() {
            w.update(equipper, |eqp: &mut Equipment<T>| {
                eqp.remove(item);
                e.item_slot = None;
            });
        }
    });
}*/
*/

//buffs applied/removed UPON adding/removing to inventory
#[derive(Clone)]
pub struct ActiveEffect {
    buffs: Vec<(Stat, i32)>,
}
impl Component for ActiveEffect {
    type Storage = VecStorage<Self>;
}
impl ActiveEffect {
    pub fn new(buffs: Vec<(Stat, i32)>) -> ActiveEffect {
        ActiveEffect{buffs}
    }
}

#[derive(Clone)]
pub struct Stackable {
    quantity: u32,
}
impl Component for Stackable {
    type Storage = VecStorage<Self>;
}

pub fn init(w: &mut GameState) {
    w.register_component::<Inventory>();
    w.register_component::<Consumable>();
    w.register_component::<ActiveEffect>();
    w.register_component::<Stackable>();
}
pub fn add_item(w: &GameState, entity: Entity, item: Entity) {
    w.update(entity, |inv: &mut Inventory| {
        if w.has_flag::<Position>(item) {
            w.delete::<Position>(item);
        }
        inv.items.push(item);
        w.update(item, |a: &mut ActiveEffect| {
            if !w.has_flag::<Name>(item) {
                println!("ActiveEffect items must have Name");
                return;
            }
            for buff in &a.buffs {
                w.update(item, |Name(name)| {
                    stats::buff(w, entity, buff.0, name, buff.1);
                });
            }
        });
    });
}
pub fn remove_item(w: &GameState, entity: Entity, item: Entity) {
    w.update(entity, |inv: &mut Inventory| {
        for i in 0..inv.items.len() {
            if inv.items[i] == item {
                inv.items.remove(i);
                if !w.has_flag::<Name>(item) {
                    println!("ActiveEffect items must have Name");
                    return;
                }
                w.update(item, |_: &mut ActiveEffect| {
                    w.update(item, |Name(name)| {
                        stats::unbuff(w, entity, name);
                    });
                });
            }
        }
        if let Some(pos) = w.clone::<Position>(entity) {
            w.insert(item, pos);
        }
    });
}

pub fn consume(w: &GameState, entity: Entity, item: Entity) {
    w.update(item, |c: &mut Consumable| {
        println!("buffing");
        for buff in &c.buffs {
            w.update(item, |Name(name)| {
                stats::buff(w, entity, buff.0, name, buff.1);
            });
        }

        println!("checking stack");
        if w.has_flag::<Stackable>(item) {
            w.update(item, |s: &mut Stackable|{
                s.quantity -= 1;
                if s.quantity <= 0 {
                    println!("deleting");
                    remove_item(w, entity, item);
                    w.delete_entity(item);
                }
            });
        } else {
            println!("deleting");
            remove_item(w, entity, item);
            w.delete_entity(item);
        }
    });
}