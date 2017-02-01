extern crate rand;

use util::{Point, Bound, Contains};
use self::rand::Rng;

pub trait MovementComponent {
  fn update(&self, Point) -> Point;
}

pub struct RandomMovementComponent {
  window_bounds: Bound
}

impl RandomMovementComponent {
  pub fn new(bound: Bound) -> RandomMovementComponent {
    RandomMovementComponent { window_bounds: bound }
  }
}

impl MovementComponent for RandomMovementComponent {
  fn update(&self, point: Point) -> Point {
    let mut offset = Point { x: point.x, y: point.y };
    let offset_x = rand::thread_rng().gen_range(0, 3i32) - 1;

    match self.window_bounds.contains(offset.offset_x(offset_x)) {
        Contains::DoesContain => offset = offset.offset_x(offset_x),
        Contains::DoesNotContain => { return point; },
    }

    let offset_y = rand::thread_rng().gen_range(0, 3i32) - 1;
    match self.window_bounds.contains(offset.offset_y(offset_y)) {
        Contains::DoesContain => offset = offset.offset_y(offset_y),
        Contains::DoesNotContain => { return point;},
    }

    offset
  }
}