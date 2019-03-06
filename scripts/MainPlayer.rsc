[Vars]
center: Position = Position{x: 200.0, y: 200.0};
angle: f64 = 0.0;
radius: f64 = 100.0;

[Components]
sprite: RenderInfo = RenderInfo(Sprites::Player);
pos: Position = Position{x: 0.0, y: 0.0};
vel: Velocity = Velocity{x: 0.0, y: 0.0};
inventory: inventory::Inventory = inventory::Inventory::new();

[Behaviour]
fn new(vars) {}

fn update(vars) {
    pos.x = vars.center.x + vars.angle.cos() * vars.radius;
    pos.y = vars.center.y + vars.angle.sin() * vars.radius;
    vars.angle += 0.1;
    if vars.angle > 360.0 {
    	vars.angle = 0.0;
    }
}