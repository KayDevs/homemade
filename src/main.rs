//use crate::combat::{Weapon};

use world::{GameState, SystemRunner, Entity, Component};
use world::storage::{HashMapStorage, BTreeMapStorage};
use homemade::common::{Name, Position, RenderInfo};
use homemade::inventory;
use homemade::stats;

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


fn main() {

    let sdl_context = sdl2::init().expect("FATAL: Could not initialize SDL2");
    let video = sdl_context.video().expect("FATAL: Could not initialize video subsystem");
    let window = video.window("rust-sdl2 demo", 640, 400)
    .position_centered()
    .fullscreen_desktop()
    .build()
    .expect("FATAL: Could not create window");

    let mut canvas = window.into_canvas().present_vsync().build().expect("FATAL: Could not create canvas");
    canvas.set_logical_size(640, 400).expect("FATAL: Could not set resolution");

    let mut cursor_s = sdl2::surface::Surface::load_bmp_rw(&mut sdl2::rwops::RWops::from_bytes(include_bytes!("resources/cursor.bmp")).expect("FATAL: Could not load resources/cursor.bmp")).expect("FATAL: Could not create surface");
    let _ = cursor_s.set_color_key(true, sdl2::pixels::Color::RGB(255, 0, 255));
    let cursor = sdl2::mouse::Cursor::from_surface(&cursor_s, 0, 0).expect("FATAL: Could not create cursor");
    cursor.set();
    //sdl_context.mouse().show_cursor(false);

    let mut w = GameState::new();
    w.register_component::<Position>();
    w.register_component::<Velocity>();
    w.register_component::<Enemy>();
    w.register_component::<Player>();
    w.register_component::<Name>();
    w.register_component::<RenderInfo>();
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
        w.insert(e, inventory::Consumable::new(vec![(stats::VITALITY, 3)]));
        w.insert(e, inventory::ActiveEffect::new(vec![(stats::VITALITY, -3)]));
        if i == 9 {
            inventory::add_item(&w, p, e);
            println!("{:?}", w.get_value::<inventory::Inventory>(p).items);
            println!("should be 29: {}", stats::get_max(&w, p, stats::VITALITY));
            //inventory::remove_item(&w, p, e);
            inventory::consume(&w, p, e);
            w.insert(e, RenderInfo("player"));
            println!("{:?}", w.get_value::<inventory::Inventory>(p).items);
            println!("should be 35: {}", stats::get_max(&w, p, stats::VITALITY));
        }
    }

    let mut event_pump = sdl_context.event_pump().expect("FATAL: Could not get events");
    'running: loop {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(60, 44, 56));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        
        chase_player(&w, p);
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
        for &i in w.iter() {
            if w.is_deleted(i) { //this logic should be integral to how `world` works; will move soon
                continue;
            }
            if let Some(Position{x, y}) = w.get(i) {
                let mut color = sdl2::pixels::Color::RGB(0, 0, 0);
                let mut rect = sdl2::rect::Rect::new(x as i32, y as i32, 16, 16);
                match w.get::<RenderInfo>(i) {
                    Some(RenderInfo("enemy")) => {
                        color = sdl2::pixels::Color::RGB(255, 0, 0);
                    }
                    Some(RenderInfo("player")) => {
                        color = sdl2::pixels::Color::RGB(0, 255, 0);
                        rect.set_width(32);
                        rect.set_height(32);
                    }
                    _ => {},
                }

                canvas.set_draw_color(color);
                let _ = canvas.fill_rect(rect);
            }
        }
        canvas.present();
        //std::thread::sleep(std::time::Duration::from_secs(2));
    }

    println!("Goodbye!!");
}