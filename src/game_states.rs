extern crate core;

use std::rc::Rc;
use std::cell::RefCell;

use rendering::windows::Windows;
use rendering::renderers::RenderingComponent;
use map::Maps;
use game::MoveInfo;
use input::GameKey::{SpecialKey};
use input::{GameKeyCode};
use util::Point;

use self::core::ops::Deref;

pub trait GameState {
  fn enter(&self, &mut Windows) {}
  fn exit(&self, &mut Windows) {}

  fn should_update_state(&self) -> bool;

  fn update(&mut self, maps: &mut Maps, windows: &mut Windows, Rc<RefCell<MoveInfo>>);

  fn render(&mut self, renderer: &mut Box<RenderingComponent>, maps: &mut Maps, windows: &mut Windows) {
    renderer.before_render_new_frame();
    for window in windows.all_windows().iter_mut() {
      renderer.attach_window(*window);
    }
    maps.render(renderer);
    renderer.after_render_new_frame();
  }
}

pub struct MovementGameState;
impl MovementGameState {
  pub fn new() -> MovementGameState {
    MovementGameState
  }
}

impl GameState for MovementGameState {
  fn should_update_state(&self) -> bool {
    true
  }

  fn update(&mut self, maps: &mut Maps, windows: &mut Windows, move_info: Rc<RefCell<MoveInfo>>) {
    let last_keypress = {
      move_info.borrow().deref().last_keypress
    };
    match last_keypress {
      Some(ks) => {
        match ks.key {
          // Because Shift is used for attack keys we don't want to do
          // anything when it's pushed. We can check for shift when we
          // process the next keypress
          SpecialKey(GameKeyCode::Shift) => {},
          _ => {
            maps.update(windows);
          }
        }
      },
      _ => {}
    }
  }
}

pub struct AttackInputGameState {
  should_update_state: bool,
  pub weapon: String
}
impl AttackInputGameState {
  pub fn new() -> AttackInputGameState {
    AttackInputGameState {
      should_update_state: false,
      weapon: "".to_string()
    }
  }
}

impl GameState for AttackInputGameState {
  fn should_update_state(&self) -> bool {
    self.should_update_state
  }

  fn enter(&self, windows: &mut Windows) {
    windows.input.flush_buffer();
    let mut msg = "Which direction do you want to attack with ".to_string();
    msg.push_str(&self.weapon);
    msg.push_str("? [Use the arrow keys to answer]");
    windows.input.buffer_message(&msg);
  }

  fn update(&mut self, maps: &mut Maps, windows: &mut Windows, move_info: Rc<RefCell<MoveInfo>>) {
    let last_keypress = {
      move_info.borrow().deref().last_keypress
    };
    match last_keypress {
      Some(ks) => {
        let mut msg = "You attack ".to_string();
        let char_point = {
          move_info.borrow().deref().char_location.clone()
        };
        let mut point = Point::new(0, 0);
        match ks.key {
          SpecialKey(GameKeyCode::Up) => {
            point = char_point.offset_y(-1);
            msg.push_str("up");
            self.should_update_state = true;
          },
          SpecialKey(GameKeyCode::Down) => {
            point = char_point.offset_y(1);
            msg.push_str("down");
            self.should_update_state = true;
          },
          SpecialKey(GameKeyCode::Left) => {
            point = char_point.offset_x(-1);
            msg.push_str("left");
            self.should_update_state = true;
          },
          SpecialKey(GameKeyCode::Right) => {
            point = char_point.offset_x(1);
            msg.push_str("right");
            self.should_update_state = true;
          },
          _ => {}
        }

        if self.should_update_state {
          match maps.enemy_at(point) {
            Some(_) => {
              msg.push_str(" with your ");
              msg.push_str(&self.weapon);
              msg.push_str("!");
              windows.messages.buffer_message(&msg);
            },
            None => {
              windows.messages.buffer_message("No enemy in that direction!");
            }
          }
        }
      },
      _ => {},
    }
  }

  fn exit(&self, windows: &mut Windows) {
    windows.input.flush_buffer();
  }
}