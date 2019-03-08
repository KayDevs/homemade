[Vars]
center: Position = Position{x: 200.0, y: 200.0};
angle: f64 = 0.0;
radius: f64 = 100.0;

[Components]
sprite: RenderInfo = RenderInfo(Sprites::Player);
pos: Position = Position{x: 0.0, y: 0.0};

[Behaviour]
fn new(vars, world) {}

fn update(vars, world) {
    pos.x = vars.center.x + vars.angle.to_radians().cos() * vars.radius;
    pos.y = vars.center.y + vars.angle.to_radians().sin() * vars.radius;
    vars.angle += 5.0;
    if vars.angle > 360.0 {
    	vars.angle = 0.0;
    }
}