use util::{Point, Bound};
use game::Game;
use rendering::renderers::RenderingComponent;
use rendering::windows::Windows;
use movement::{RandomMovementComponent, MovementComponent, UserMovementComponent, AggroMovementComponent};


pub struct Actor {
    pub position: Point,
    display_char: char,
    movement_component: Box<MovementComponent>,
    is_pc: bool
}

impl Clone for Actor {
    fn clone(&self) -> Actor {
        let mc = self.movement_component.box_clone();
        Actor::new(self.position.x, self.position.y, self.display_char, mc, self.is_pc)
    }
}

impl Actor {
    pub fn new(x: i32, y: i32, dc: char, mc: Box<MovementComponent>, is_pc: bool) -> Actor {
        Actor {
            position: Point {x: x, y: y},
            display_char: dc,
            movement_component: mc,
            is_pc: is_pc
         }
    }

    pub fn update(&mut self, windows: &mut Windows) {
        self.position = self.movement_component.update(self.position, windows);
    }

    pub fn render(&self, rendering_component: &mut Box<RenderingComponent>) {
        rendering_component.render_object(self.position, self.display_char);
    }

    pub fn dog(x: i32, y: i32, bound: Bound) -> Actor {
        let mc: Box<MovementComponent> = Box::new(RandomMovementComponent::new(bound));
        Actor::new(x, y, 'd', mc, false)
    }

    pub fn cat(x: i32, y: i32, bound: Bound) -> Actor {
        let mc: Box<MovementComponent> = Box::new(RandomMovementComponent::new(bound));
        Actor::new(x, y, 'c', mc, false)
    }

    pub fn heroine(bound: Bound) -> Actor {
        let point = Game::get_character_point();
        let mc: Box<MovementComponent> = Box::new(UserMovementComponent::new(bound));
        Actor::new(point.x, point.y, '@', mc, true)
    }

    pub fn kobold(x: i32, y: i32, bound: Bound) -> Actor {
        let mc: Box<MovementComponent> = Box::new(AggroMovementComponent::new(bound));
        Actor::new(x, y, 'k', mc, false)
    }
}