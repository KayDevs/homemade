use std::collections::HashMap;
use world::{GameState, Entity, Component};
use world::storage::VecStorage;

pub use self::Stat::*; //just so nobody has to type 'Stat' again for the enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum Stat {
    VITALITY,       //max hp
    STRENGTH,       //max atk
    CONSTITUTION,   //max def
    INTELLIGENCE,   //max sp. atk
    WISDOM,         //max sp. def
    DEXTERITY,      //max agility
}

//not really a stat, but related
#[derive(Clone)]
pub struct Health(pub i32); //current hp; max determined by Vitality
impl Component for Health {
    type Storage = VecStorage<Self>;
}

#[derive(Clone)]
struct Stats(HashMap<Stat, HashMap<Option<&'static str>, i32>>);
impl Component for Stats {
    type Storage = VecStorage<Self>;
}
impl Stats {
    fn new() -> Stats {
        Stats(HashMap::new())
    }
}

pub fn init(w: &mut GameState) {
    w.register_component::<Stats>();
    w.register_component::<Health>();
}


pub fn set_base(w: &GameState, i: Entity, stat: Stat, value: i32) {
    match w.clone::<Stats>(i) {
        Some(Stats(ref mut stats)) => {
            let s = stats.entry(stat).or_insert(HashMap::new());
            s.insert(None, value);
        },
        None => {
            w.insert(i, Stats::new());
            w.update(i, |Stats(ref mut stats)| {
                let s = stats.entry(stat).or_insert(HashMap::new());
                s.insert(None, value);
            });
        }
    }

    //special cases for associated values
    match stat {
        VITALITY => w.insert(i, Health(value)),
        _ => {}
    }
}

/* 
IF the entity does not have stats at all,
or it does not have the stat you're looking for,
or it does not have a base stat for the stat you're looking for,
then this function will return zero.
*/ 
pub fn get_base(w: &GameState, i: Entity, stat: Stat) -> i32 {
    if let Some(Stats(stats)) = w.clone(i) {
        if let Some(s) = stats.get(&stat) {
            if let Some(s) = s.get(&None) {
                return s.clone();
            }
        }
    }
    return 0;
}
//gets sum of base stat + all buffs/debuffs
pub fn get_max(w: &GameState, i: Entity, stat: Stat) -> i32 {
    if let Some(Stats(stats)) = w.clone(i) {
        if let Some(s) = stats.get(&stat) {
            let mut base = 0;
            for i in s.values() {
                base += i;
            }
            return base;
        }
    }
    0
}

//if the stat has an associated realtime value, i.e. HP for Vitality, 
//then operate on that value
//otherwise operate on the stat itself
//if it doesn't even have the stat, return 0 and don't do anything
pub fn get(w: &GameState, i: Entity, stat: Stat) -> i32 {
    if let Some(Stats(stats)) = w.clone(i) {
        match stat {
            VITALITY => {
                if let Some(Health(hp)) = w.clone::<Health>(i) {
                    return hp;
                }
            }
            _ => {
                if let Some(s) = stats.get(&stat) {
                    let mut base = 0;
                    for i in s.values() {
                        base += i;
                    }
                    return base;
                }
            }
        }
    }
    0
}
pub fn set(w: &GameState, i: Entity, stat: Stat, value: i32) {
    match stat {
        VITALITY => {
            w.update(i, |Health(ref mut hp)| {
                *hp = value;
            });
        },
        _ => {
            set_base(w, i, stat, value);
        }
    }
}
pub fn modify(w: &GameState, i: Entity, stat: Stat, amount: i32) {
    match stat {
        VITALITY => {
            w.update(i, |Health(ref mut hp)| {
                *hp += amount;
            });
        }
        _ => {
            set_base(w, i, stat, get_base(w, i, stat) + amount);
        }
    }
}

//note: these functions won't work if the entity doesn't have a base stat
pub fn buff(w: &GameState, i: Entity, stat: Stat, name: &'static str, buff: i32) {
    w.update(i, |Stats(ref mut stats)| {
        let s = stats.entry(stat).or_insert(HashMap::new());
        s.insert(Some(name), buff);
    });

    //special cases for associated values
    match stat {
        VITALITY => {
            //in the case that Vitality is buffed down to lower than current HP,
            //then subtract from HP as well
            if get_max(w, i, stat) < get(w, i, stat) {
                set(w, i, stat, get_max(w, i, stat));
            }
        }
        _ => {}
    }
}
pub fn unbuff(w: &GameState, i: Entity, name: &'static str) {
    w.update(i, |Stats(ref mut stats)| {
        for s in stats.values_mut() {
            s.remove(&Some(name));
        }
    });
}


//make sure everything works right
#[cfg(test)]
mod tests {
    use super::*;

    fn prepare_world() -> GameState {
        let mut w = GameState::new();
        init(&mut w);
        w
    }

    #[test]
    fn base_stats() {
        let mut w = prepare_world();
        let stats_test_entity = w.create_entity();
        set_base(&w, stats_test_entity, VITALITY, 32);
        assert_eq!(get_base(&w, stats_test_entity, VITALITY), 32); 
        w.delete_entity(stats_test_entity);
    }
    #[test]
    fn modify_value() {
        let mut w = prepare_world();
        let stats_test_entity = w.create_entity();
        set_base(&w, stats_test_entity, VITALITY, 32);
        assert_eq!(get_base(&w, stats_test_entity, VITALITY), 32); 
        modify(&w, stats_test_entity, VITALITY, -3);
        //check that health got lowered but vitality stayed the same
        assert_eq!(get(&w, stats_test_entity, VITALITY), 29);
        assert_eq!(get_base(&w, stats_test_entity, VITALITY), 32);
        w.delete_entity(stats_test_entity);
    }
    #[test]
    fn buff_base() {
        let mut w = prepare_world();
        let stats_test_entity = w.create_entity();
        set_base(&w, stats_test_entity, VITALITY, 32);
        buff(&w, stats_test_entity, VITALITY, "health buff 1", -5);
        assert_eq!(get_base(&w, stats_test_entity, VITALITY), 32);
        assert_eq!(get_max(&w, stats_test_entity, VITALITY), 27);
        assert_eq!(get(&w, stats_test_entity, VITALITY), 27);
        w.delete_entity(stats_test_entity);
    }
    #[test]
    fn buff_vitality() {
        let mut w = prepare_world();
        let stats_test_entity = w.create_entity();
        set_base(&w, stats_test_entity, VITALITY, 32);
        buff(&w, stats_test_entity, VITALITY, "health buff 1", -5);
        //make sure that after a debuff her health stays lowered
        unbuff(&w, stats_test_entity, "health buff 1");
        assert_eq!(get(&w, stats_test_entity, VITALITY), 27);
        w.delete_entity(stats_test_entity);
    }
}