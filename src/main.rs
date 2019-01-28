//use crate::combat::{Weapon};

use world::{GameState, SystemRunner, Entity, Component};
use world::storage::{HashMapStorage, BTreeMapStorage};
use homemade::common;
use homemade::common::{Name, Position, RenderInfo};
use homemade::inventory;
use homemade::stats;
use std::error::Error;
use resources::{Resources, Sprites};

#[derive(Clone)]
struct Player;
impl Component for Player {
    type Storage = BTreeMapStorage<Self>;
}

/*mod combat {
    use world::Component;
    use world::storage::VecStorage;
    #[derive(Clone, Debug)]
    pub struct Weapon {
        pub range: u32, //short for swords, longer for arrows
        pub damage: i32, //pos for damage, neg for healing, 0 for ineffectual
    }
    impl Component for Weapon {
        type Storage = VecStorage<Self>;
    }
}*/

#[derive(Clone)]
struct Enemy;
impl Component for Enemy {
    type Storage = BTreeMapStorage<Self>;
}

fn chase_player(w: &GameState, p: Entity) {
    let player_pos = w.get_value::<Position>(p);

    w.update_all(|i, enemy_pos: &mut Position| {
        if w.has_flag::<Enemy>(i) {
            let diffx = enemy_pos.x - player_pos.x;
            let diffy = enemy_pos.y - player_pos.y;
            if diffx.abs() > 0.0 {
                if diffx > 0.0 {
                    enemy_pos.x -= 1.0;
                } else {
                    enemy_pos.x += 1.0;
                }
            }
            if diffy.abs() > 0.0 {
                if diffy > 0.0 {
                    enemy_pos.y -= 1.0;
                } else {
                    enemy_pos.y += 1.0;
                }
            }
        }
    });
}

#[derive(Clone, Copy, Default, Debug)]
struct Velocity {
    x: f64,
    y: f64,
}
impl Component for Velocity {
    type Storage = HashMapStorage<Self>;
}


//include all the static resources from codegen
include!(concat!(env!("OUT_DIR"), "/resources.rs"));

fn main() -> Result<(), Box<Error>> {
    let sdl_context = sdl2::init()?;
    let video = sdl_context.video()?;
    let window = video.window("rust-sdl2 demo", 640, 400)
    .position_centered()
    .fullscreen_desktop()
    .build()?;

    let mut canvas = window.into_canvas().present_vsync().build()?;
    canvas.set_logical_size(640, 400)?;
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

    sdl_context.mouse().show_cursor(false);

    let mut r = Resources::new(&canvas)?;

    let mut w = GameState::new();

    w.register_component::<Velocity>();
    w.register_component::<Enemy>();
    w.register_component::<Player>();
    common::init(&mut w);
    stats::init(&mut w);
    inventory::init(&mut w);
    //w.register_component::<Weapon>();
    //w.register_component::<Equippable<Weapon>>(); //containee
    //w.register_component::<Equipment<Weapon>>(); //container
    let p = w.create_entity();
    w.insert(p, Player);
    w.insert(p, Position{x: 0.0, y: 0.0});
    w.insert(p, Velocity{x: 2.0, y: -2.0});
    w.insert(p, Name("kay"));
    w.insert(p, RenderInfo("player"));
    stats::set_base(&w, p, stats::VITALITY, 32);
    w.insert(p, inventory::Inventory::new());

    //w.insert(p, Equipment::<Weapon>::new(3));
    /*let sword = w.create_entity();
    w.insert(sword, Weapon{damage: 2, range: 1});
    w.insert(sword, Equippable::<Weapon>::new());
    w.insert(sword, Name("the flaming raging poisoning sword of doom"));
    w.insert(p, Inventory::new());
    w.update(p, |inv: &mut Inventory|{
        inv.add_item(&w, sword);
        //inv.add_item(&w, p);
        for &i in inv.items() {
            println!("{}: {}", i, w.get_value::<Name>(i).0);
        }
    });
    //inventory::equip::<Weapon>(&w, p, sword);
    println!("{:?}", w.get_value::<Equipment<Weapon>>(p).equipment());
    //inventory::unequip::<Weapon>(&w, p, sword);
    println!("{:?}", w.get_value::<Equipment<Weapon>>(p).equipment());
    */

    for i in 0..10 {
        let e = w.create_entity();
        w.insert(e, Enemy);
        w.insert(e, Position{x: f64::from(i) * 32.0, y: 10.0});
        w.insert(e, Name("enemy"));
        w.insert(e, RenderInfo("enemy"));
    }


    //TODO: move this into 'tests' mod of 'inventory'
    let e = w.create_entity();
    w.insert(e, RenderInfo("enemy"));
    w.insert(e, Name("Inventory Test Entity"));
    w.insert(e, Position{x: 200.0, y: 300.0});
    w.insert(e, inventory::Consumable::new(vec![(stats::VITALITY, 3)]));
    w.insert(e, inventory::ActiveEffect::new(vec![(stats::VITALITY, -3)]));
    inventory::add_item(&w, p, e);
    println!("{:?}", w.get_value::<inventory::Inventory>(p).items);
    println!("should be 29: {}", stats::get_max(&w, p, stats::VITALITY));
    //inventory::remove_item(&w, p, e);
    inventory::consume(&w, p, e);
    println!("{:?}", w.get_value::<inventory::Inventory>(p).items);
    println!("should be 35: {}", stats::get_max(&w, p, stats::VITALITY));
    

    println!("こんにしわ! starting main loop");
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        //this makes the letterboxing black on screens with different resolutions
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        //this is the actual background color
        canvas.set_draw_color(sdl2::pixels::Color::RGB(60, 44, 56));
        let _ = canvas.fill_rect(None);

        //parse events
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown{keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                _ => {}
            }
        }
        
        //using `update` syntax in function
        chase_player(&w, p);
        //using new `run` SystemRunner syntax
        w.run(|(_, pos, vel): (&mut Player, &mut Position, &mut Velocity)| {
            if pos.x + 32.0 > 640 as f64 {
                vel.x *= -1.0;
            }
            if pos.x < 0 as f64 {
                vel.x *= -1.0;
            }
            if pos.y + 32.0 > 400 as f64 {
                vel.y *= -1.0;
            }
            if pos.y < 0 as f64 {
                vel.y *= -1.0;
            }
            pos.x += vel.x;
            pos.y += vel.y;
        });
        
        //rendering system :3
        //TODO: animation system, render according to seconds 
        // maybe store a start_time on every .reset() and then do current_frame = (seconds_passed - start_time) % num_frames;
        //TODO: rendering system, render according to physical units and not pixels
        use sdl2::rect::Rect;
        w.update_all(|e, &mut Position{x, y}| {
            let mut rect = Rect::new(x as i32, y as i32, 16, 16);
            w.update(e, |&mut RenderInfo(info)| {
                match info {
                    "enemy" => {
                        r[Sprites::Enemy].set_alpha_mod(127);
                        let _ = canvas.copy(&r[Sprites::Enemy], None, rect);
                        r[Sprites::Enemy].set_alpha_mod(255);
                    }
                    "player" => {
                        rect.set_width(32);
                        rect.set_height(32);
                        let _ = canvas.copy(&r[Sprites::Player], None, rect);
                    }
                    _ => {}
                }
            });
        });

        let _ = canvas.copy(&r[Sprites::Cursor], None, sdl2::rect::Rect::new(event_pump.mouse_state().x(), event_pump.mouse_state().y(), 16, 16));

        canvas.present();
        //std::thread::sleep(std::time::Duration::from_secs(2));
    }

    println!("Goodbye!!");
    Ok(())
}