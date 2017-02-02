extern crate rand;
extern crate tcod;

use util::{Point, Bound, Contains};
use game::Game;
use self::rand::Rng;

use self::tcod::input::KeyCode::{Up, Down, Left, Right};
use self::tcod::input::Key;

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

pub struct TcodUserMovementComponent {
  window_bounds: Bound
}

impl TcodUserMovementComponent {
  pub fn new(bound: Bound) -> TcodUserMovementComponent {
    TcodUserMovementComponent { window_bounds: bound }
  }
}

impl MovementComponent for TcodUserMovementComponent {
  fn update(&self, point: Point) -> Point {
    let mut offset = Point { x: point.x, y: point.y };
    offset = match Game::get_last_keypress() {
      Some(keypress) => {
        match keypress {
          Key {code: Up, .. } => {
            offset.offset_y(-1)
          },
          Key {code: Down, .. } => {
            offset.offset_y(1)
          },
          Key {code: Left, .. } => {
            offset.offset_x(-1)
          },
          Key {code: Right, .. } => {
            offset.offset_x(1)
          },
          _ => offset
        }
      },
      None => offset
    };

    match self.window_bounds.contains(offset) {
      Contains::DoesContain => { offset }
      Contains::DoesNotContain => point
    }
  }
}