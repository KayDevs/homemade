[Vars]
followers: Vec<Entity> = Vec::new();
player: Option<Entity> = None;

[Components]
sprite: RenderInfo = RenderInfo(Sprites::Enemy);
pos: Position = Position{x: 0.0, y: 0.0};
enemy: Enemy = Enemy;
name: Name = Name("enemy");

[Behaviour]
fn new(vars, world) {
	for i in 0..10 {
		let f = Follower::new(&world);
		vars.followers.push(f);
	}
	world.read_all(|p, _: &Player| {
		vars.player = Some(p);
		println!("player id: {}", p.id());
	});
}

fn update(vars, world) {
	if let Some(player) = vars.player {
		if let Some(player_pos) = world.clone::<Position>(player) {
			println!("{}, {}", player_pos.x, player_pos.y);
			follow(0.0, &player_pos, pos);
	    }
	}
	world.update(vars.followers[0], |fpos: &mut Position|{
		follow(16.0, pos, fpos);	
	}); 
	for i in 1..10 {
		if let Some(lpos) = world.clone::<Position>(vars.followers[i - 1]) {
			world.update(vars.followers[i], |fpos: &mut Position| {
				follow(16.0, &lpos, fpos);	
			});
		}
	}
}