use movement::MovementComponent;

use traits::Updates;
use util::Point;
use rendering::RenderingComponent;

pub struct NPC {
    position: Point,
    display_char: char,
    movement_component: Box<MovementComponent>
}

impl NPC {
    pub fn new(x: i32, y: i32, dc: char, mc: Box<MovementComponent>) -> NPC {
        NPC { position: Point { x: x, y: y }, display_char: dc, movement_component: mc }
    }
}

impl Updates for NPC {
    fn update(&mut self) {
       self.position = self.movement_component.update(self.position);
    }

    fn render(&self, rendering_component: &mut Box<RenderingComponent>) {
        rendering_component.render_object(self.position, self.display_char);
    }
}