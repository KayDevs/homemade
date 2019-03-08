//some common components (VecStorage-level common)

extern crate world;
use world::{GameState, Component, SystemRunner};
use world::storage::{VecStorage, HashMapStorage};

#[derive(Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}
impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Clone)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}
impl Component for Velocity {
    type Storage = HashMapStorage<Self>;
}

#[derive(Clone)]
pub struct Acceleration {
	pub x: f64,
	pub y: f64,
}
impl Component for Acceleration {
	type Storage = HashMapStorage<Self>;
}


//friction should be applied per-object by environmental objects
//i.e. "entity steps on ice" -> insert(entity, Friction(0.5))
//     "entity steps off ice" -> remove::<Friction>(entity)
//UNLESS the entity has it explicitly set (i.e. it's hovering; f=1.0)
#[derive(Clone)]
pub struct Friction {
	pub x: f64,
	pub y: f64,
}
impl Component for Friction {
	type Storage = HashMapStorage<Self>;
}


#[allow(unused)]
pub fn run_physics(w: &GameState) {
	w.run(|(vel, acc): (&mut Velocity, &mut Acceleration)| {
		vel.x += acc.x;
		vel.y += acc.y;
	});
	w.run(|(pos, vel): (&mut Position, &mut Velocity)| {
		pos.x += vel.x;
		pos.y += vel.y;
	});
	w.update_all(|i, vel: &mut Velocity| {
		if let Some(fric) = w.clone::<Friction>(i) {
			vel.x *= fric.x;
			vel.y *= fric.y;
		} else {
			vel.x = 0.0; //implicitly halting friction if not specified otherwise
			vel.y = 0.0;
		}
	});
	//note: in things that contain stats, max velocity is determined by Dexterity
}

//for indexing
#[derive(Clone)]
pub struct Name(pub &'static str);
impl Component for Name {
    type Storage = VecStorage<Self>;
}

pub fn init(w: &mut GameState) {
    w.register_component::<Position>();
    w.register_component::<Velocity>();
    w.register_component::<Acceleration>();
    w.register_component::<Friction>();
    w.register_component::<Name>();
}