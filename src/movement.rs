extern crate rand;

use util::{Point, Bound, Contains, XPointRelation, YPointRelation, PointEquality};
use game::Game;
use rendering::windows::Windows;
use self::rand::Rng;

use input::GameKey::{SpecialKey};
use input::GameKeyCode;

pub trait MovementComponent {
  fn update(&self, Point, &mut Windows) -> Point;
  fn box_clone(&self) -> Box<MovementComponent>;
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
  fn update(&self, point: Point, _: &mut Windows) -> Point {
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

  fn box_clone(&self) -> Box<MovementComponent> {
    Box::new(RandomMovementComponent { window_bounds: self.window_bounds })
  }
}

pub struct UserMovementComponent {
  window_bounds: Bound
}

impl UserMovementComponent {
  pub fn new(bound: Bound) -> UserMovementComponent {
    UserMovementComponent { window_bounds: bound }
  }
}

impl MovementComponent for UserMovementComponent {
  fn update(&self, point: Point, windows: &mut Windows) -> Point {
    let mut offset = Point { x: point.x, y: point.y };
    let last_keypress = {
      self.move_info.borrow().deref().last_keypress
    }
    offset = match last_keypress {
      Some(keypress) => {
        match keypress.key {
          SpecialKey(GameKeyCode::Up) => {
            offset.offset_y(-1)
          },
          SpecialKey(GameKeyCode::Down) => {
            offset.offset_y(1)
          },
          SpecialKey(GameKeyCode::Left) => {
            offset.offset_x(-1)
          },
          SpecialKey(GameKeyCode::Right) => {
            offset.offset_x(1)
          },
          _ => offset
        }
      },
      None => offset
    };

    match self.window_bounds.contains(offset) {
      Contains::DoesContain => { offset }
      Contains::DoesNotContain => {
        windows.messages.buffer_message("You can't move that way!");
        point
      }
    }
  }

  fn box_clone(&self) -> Box<MovementComponent> {
    Box::new(UserMovementComponent { window_bounds: self.window_bounds })
  }
}

pub struct AggroMovementComponent {
  window_bounds: Bound
}

impl AggroMovementComponent {
  pub fn new(bound: Bound) -> AggroMovementComponent {
    AggroMovementComponent { window_bounds: bound }
  }
}

impl MovementComponent for AggroMovementComponent {
  fn update(&self, point: Point, _: &mut Windows) -> Point {
    let char_point = {
      move_info.borrow().deref().char_location;
    }
    let mut offset = Point { x: 0, y: 0 };

    match point.compare_x(char_point) {
      XPointRelation::RightOfPoint => offset = offset.offset_x(-1),
      XPointRelation::LeftOfPoint => offset = offset.offset_x(1),
      XPointRelation::OnPointX => {}
    }

    match point.compare_y(char_point) {
      YPointRelation::BelowPoint => offset = offset.offset_y(-1),
      YPointRelation::AbovePoint => offset = offset.offset_y(1),
      YPointRelation::OnPointY => {}
    }

    match point.offset(offset).compare(char_point) {
      PointEquality::PointsEqual => point,
      PointEquality::PointsNotEqual  => {
        match self.window_bounds.contains(point.offset(offset)) {
          Contains::DoesContain => point.offset(offset),
          Contains::DoesNotContain => point
        }
      }
    }
  }

  fn box_clone(&self) -> Box<MovementComponent> {
    Box::new(AggroMovementComponent { window_bounds: self.window_bounds })
  }
}
