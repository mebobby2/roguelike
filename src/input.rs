extern crate tcod;

use self::tcod::input::Key;
use self::tcod::input::KeyCode::{Up, Down, Left, Right, Number6, Number8, Number5, Shift, Escape};

#[derive(Copy, Clone)]
pub enum GameKey {
    Printable(char),
    SpecialKey(GameKeyCode)
}

#[derive(Copy, Clone)]
pub struct KeyboardInput {
  pub key: GameKey
}

#[derive(Copy, Clone)]
pub enum GameKeyCode {
    // Arrow keys
    Up,
    Down,
    Left,
    Right,

    // Special
    Shift,
    Escape,

    // Default
    None
}

pub trait InputComponent<T> {
  fn translate_input(&self, T) -> KeyboardInput;
}

pub struct TcodInputComponent;
impl TcodInputComponent {
  pub fn new() -> TcodInputComponent { TcodInputComponent }
}

impl InputComponent<Key> for TcodInputComponent {
  fn translate_input(&self, k: Key) -> KeyboardInput {
    let key: GameKey = if k.shift {
      match k {
        Key { code: Number5, ..  } => GameKey::Printable('%'),
        Key { code: Number6, ..  } => GameKey::Printable('^'),
        Key { code: Number8, ..  } => GameKey::Printable('%'),
        _ => GameKey::SpecialKey(GameKeyCode::None)
      }
    } else {
      match k {
        Key { printable: '/', .. } => GameKey::Printable('/'),
        Key { code: Up, .. } => GameKey::SpecialKey(GameKeyCode::Up),
        Key { code: Down, .. } => GameKey::SpecialKey(GameKeyCode::Down),
        Key { code: Left, .. } => GameKey::SpecialKey(GameKeyCode::Left),
        Key { code: Right, .. } =>  GameKey::SpecialKey(GameKeyCode::Right),
        Key { code: Shift, ..  } => GameKey::SpecialKey(GameKeyCode::Shift),
        Key { code: Escape, ..  } => GameKey::SpecialKey(GameKeyCode::Escape),
        _ => GameKey::SpecialKey(GameKeyCode::None)
      }
    };

    KeyboardInput{ key: key }
  }
}
