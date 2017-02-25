extern crate rand;
extern crate core;

use util::{Point, Bound, Contains, XPointRelation, YPointRelation, PointEquality};
use game::Game;
use rendering::windows::Windows;
use self::rand::Rng;
use game::MoveInfo;

use input::GameKey::{SpecialKey};
use input::GameKeyCode;

use std::cell::RefCell;
use std::rc::Rc;

use self::core::ops::Deref;

pub trait MovementComponent {
  fn update(&self, Point, &mut Windows) -> Point;
  fn box_clone(&self) -> Box<MovementComponent>;
}

pub struct RandomMovementComponent {
  move_info: Rc<RefCell<MoveInfo>>
}

impl RandomMovementComponent {
  pub fn new(move_info: Rc<RefCell<MoveInfo>>) -> RandomMovementComponent {
    RandomMovementComponent { move_info: move_info }
  }
}

impl MovementComponent for RandomMovementComponent {
  fn update(&self, point: Point, _: &mut Windows) -> Point {
    let mut offset = Point { x: point.x, y: point.y };
    let offset_x = rand::thread_rng().gen_range(0, 3i32) - 1;
    let bound = {
      self.move_info.borrow().deref().bounds
    };
    match bound.contains(offset.offset_x(offset_x)) {
        Contains::DoesContain => offset = offset.offset_x(offset_x),
        Contains::DoesNotContain => { return point; },
    }

    let offset_y = rand::thread_rng().gen_range(0, 3i32) - 1;
    match bound.contains(offset.offset_y(offset_y)) {
        Contains::DoesContain => offset = offset.offset_y(offset_y),
        Contains::DoesNotContain => { return point;},
    }

    offset
  }

  fn box_clone(&self) -> Box<MovementComponent> {
    Box::new(RandomMovementComponent { move_info: self.move_info })
  }
}

pub struct UserMovementComponent {
  move_info: Rc<RefCell<MoveInfo>>
}

impl UserMovementComponent {
  pub fn new(move_info: Rc<RefCell<MoveInfo>>) -> UserMovementComponent {
    UserMovementComponent { move_info: move_info }
  }
}

impl MovementComponent for UserMovementComponent {
  fn update(&self, point: Point, windows: &mut Windows) -> Point {
    let mut offset = Point { x: point.x, y: point.y };
    let last_keypress = {
      self.move_info.borrow().deref().last_keypress
    };
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

    let bound = {
      self.move_info.borrow().deref().bounds
    };

    match bound.contains(offset) {
      Contains::DoesContain => { offset }
      Contains::DoesNotContain => {
        windows.messages.buffer_message("You can't move that way!");
        point
      }
    }
  }

  fn box_clone(&self) -> Box<MovementComponent> {
    Box::new(UserMovementComponent { move_info: self.move_info })
  }
}

pub struct AggroMovementComponent {
  move_info: Rc<RefCell<MoveInfo>>
}

impl AggroMovementComponent {
  pub fn new(move_info: Rc<RefCell<MoveInfo>>) -> AggroMovementComponent {
    AggroMovementComponent { move_info: move_info }
  }
}

impl MovementComponent for AggroMovementComponent {
  fn update(&self, point: Point, _: &mut Windows) -> Point {
    let char_point = {
      self.move_info.borrow().deref().char_location
    };
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
        let bound = {
          self.move_info.borrow().deref().bounds
        };
        match bound.contains(point.offset(offset)) {
          Contains::DoesContain => point.offset(offset),
          Contains::DoesNotContain => point
        }
      }
    }
  }

  fn box_clone(&self) -> Box<MovementComponent> {
    Box::new(AggroMovementComponent { move_info: self.move_info })
  }
}
