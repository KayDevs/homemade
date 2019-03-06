//use crate::combat::{Weapon};

use world::{GameState, SystemRunner, Entity, Component};
use world::storage::{VecStorage, BTreeMapStorage};
use homemade::common;
use homemade::common::{Name, Position, Velocity, Friction};
use homemade::inventory;
use homemade::stats;
use std::error::Error;
use resources::{Resources, Sprites};
use scripts::MainPlayer;

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

//include all the static resources from codegen
include!(concat!(env!("OUT_DIR"), "/resources.rs"));
include!(concat!(env!("OUT_DIR"), "/scripts.rs"));

//inject these into the engine renderer initialization code
//invariant: make the engine run with or without these, since renderer is supposed to be independent
#[derive(Clone)]
struct RenderInfo(Sprites);
impl Component for RenderInfo {
    type Storage = VecStorage<Self>;
}

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
    let texture_creator = canvas.texture_creator();
    let mut lighting = texture_creator.create_texture_target(texture_creator.default_pixel_format(), 640, 400)?;

    sdl_context.mouse().show_cursor(false);

    let mut r = Resources::new(&canvas)?;
    let mut w = GameState::new();

    w.register_component::<RenderInfo>();

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
    w.insert(p, Velocity{x: 2.0, y: 2.0});
    w.insert(p, Friction{x: 1.0, y: 1.0});
    w.insert(p, Name("kay"));
    w.insert(p, RenderInfo(Sprites::Player));
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
        w.insert(e, RenderInfo(Sprites::Enemy));
    }


    //TODO: move this into 'tests' mod of 'inventory'
    let e = w.create_entity();
    w.insert(e, RenderInfo(Sprites::Enemy));
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


    let myp = MainPlayer::new(&mut w);
    
    println!("こんにしわ! starting main loop");
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
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

        MainPlayer::update(&w);
        println!("({}, {})", w.get_value::<Position>(myp).x, w.get_value::<Position>(myp).y);
        
        //using `update` syntax in function
        chase_player(&w, p);
        //using new `run` SystemRunner syntax
        w.run(|(_, pos, vel): (&mut Player, &mut Position, &mut Velocity)| {
            if pos.x + 32.0 > 640.0 || pos.x < 0.0 {
                vel.x *= -1.0;
            }
            if pos.y + 32.0 > 400.0 || pos.y < 0.0 {
                vel.y *= -1.0;
            }
        });

        common::run_physics(&w);
        
        //rendering system :3
        //TODO: animation system, render according to seconds 
        // maybe store a start_time on every .reset() and then do current_frame = (seconds_passed - start_time) % num_frames;
        //TODO: rendering system, render according to physical units and not pixels
        use sdl2::rect::Rect;
        use sdl2::pixels::Color;
        use sdl2::render::BlendMode;
        //this makes the letterboxing black on screens with different resolutions
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        //this is the actual background color
        canvas.set_draw_color(Color::RGB(60, 44, 56));
        let _ = canvas.fill_rect(None);

        w.update_all(|e, &mut Position{x, y}| {
            let mut rect = Rect::new(x as i32, y as i32, 16, 16);
            w.update(e, |RenderInfo(info)| {
                match info {
                    Sprites::Enemy => {
                        r[Sprites::Enemy].set_alpha_mod(127);
                        let _ = canvas.copy(&r[Sprites::Enemy], None, rect);
                        r[Sprites::Enemy].set_alpha_mod(255);
                    }
                    Sprites::Player => {
                        rect.set_width(32);
                        rect.set_height(32);
                        let _ = canvas.copy(&r[Sprites::Player], None, rect);
                    }
                    _ => {
                        //generic function idea:
                        //rect.set_width(&r[info].query().width);
                        //rect.set_height(&r[info].query().height); 
                        //let _ = canvas.copy(&r[info], None, rect);
                    }
                }
            });
        });

        let _ = canvas.with_texture_canvas(&mut lighting, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGBA(128, 0, 128, 128));
            texture_canvas.clear();
            texture_canvas.set_draw_color(Color::RGBA(255, 225, 180, 255));
            let _ = texture_canvas.fill_rect(Rect::new(20, 20, 200, 100));
            texture_canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
            let _ = texture_canvas.fill_rect(Rect::new(320, 0, 320, 400));
        });
        lighting.set_blend_mode(BlendMode::Mod);
        let _ = canvas.copy(&lighting, None, None);

        let _ = canvas.copy(&r[Sprites::Cursor], None, Rect::new(event_pump.mouse_state().x(), event_pump.mouse_state().y(), 16, 16));

        canvas.present();
        //std::thread::sleep(std::time::Duration::from_secs(2));
    }

    println!("バイ-バイ! shutting down"); //sublime doesn't render ths as monospace for some reason
    Ok(())
}