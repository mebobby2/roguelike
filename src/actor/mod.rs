use util::Point;
use rendering::RenderingComponent;
use movement::MovementComponent;

pub struct Actor {
    position: Point,
    display_char: char,
    movement_component: Box<MovementComponent>
}

impl Actor {
    pub fn new(x: i32, y: i32, dc: char, mc: Box<MovementComponent>) -> Actor {
        Actor { position: Point {x: x, y: y}, display_char: dc, movement_component: mc }
    }

    pub fn update(&mut self) {
        self.position = self.movement_component.update(self.position);
    }

    pub fn render(&self, rendering_component: &mut Box<RenderingComponent>) {
        rendering_component.render_object(self.position, self.display_char);
    }
}