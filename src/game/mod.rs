extern crate tcod;

use self::tcod::input::Key;
use self::tcod::input::KeyCode::{Up, Down, Left, Right, Number6, Number8, Number5, Shift};
use util::{Bound, Point};
use rendering::{RenderingComponent, TcodRenderingComponent, WindowComponent, TcodStatsWindowComponent, TcodInputWindowComponent, TcodMessagesWindowComponent, TcodMapWindowComponent};
use actor::Actor;

pub struct Game {
    pub exit: bool,
    pub window_bounds: Bound,
    pub rendering_component: Box<RenderingComponent>,
    pub windows: Windows,
    pub game_state: Box<GameState>
}

static mut LAST_KEYPRESS: Option<Key> = None;
static mut CHAR_LOCATION: Point = Point { x: 40, y: 25 };

impl Game {
  pub fn new() -> Game {
    let total_bounds = Bound::new(0, 0, 99, 61);
    let stats_bounds = Bound::new(79, 0, 99, 49);
    let input_bounds = Bound::new(0, 50, 99, 52);
    let message_bounds = Bound::new(0, 53, 99, 61);
    let map_bounds = Bound::new(0, 0, 78, 49);

    let rc: Box<RenderingComponent> = Box::new(TcodRenderingComponent::new(total_bounds));
    let sw: Box<WindowComponent> = Box::new(TcodStatsWindowComponent::new(stats_bounds));
    let iw: Box<WindowComponent> = Box::new(TcodInputWindowComponent::new(input_bounds));
    let mw: Box<WindowComponent> = Box::new(TcodMessagesWindowComponent::new(message_bounds));
    let maw: Box<WindowComponent> = Box::new(TcodMapWindowComponent::new(map_bounds));

    let windows = Windows {
      input: iw,
      messages: mw,
      map: maw,
      stats: sw,
    };

    let gs: Box<GameState> = Box::new(MovementGameState::new());

    Game {
      exit: false,
      window_bounds: total_bounds,
      rendering_component: rc,
      windows: windows,
      game_state: gs
    }
  }

  pub fn render(&mut self, npcs: &Vec<Box<Actor>>, c: &Actor) {
    self.game_state.render(&mut self.rendering_component, npcs, c, &mut self.windows);
  }

  pub fn update(&mut self, npcs: &mut Vec<Box<Actor>>, c: &mut Actor) {
    if self.game_state.should_update_state() {
      self.game_state.exit();
      self.update_state();
      self.game_state.enter(&mut self.windows);
    }

    self.game_state.update(npcs, c, &mut self.windows);
  }

  pub fn wait_for_keypress(&mut self) -> Key {
    let k = self.rendering_component.wait_for_keypress();
    match k {
        Key {printable: '/', .. } => self.windows.input.buffer_message("Which direction would you like to attack with your heroic sword? [Press an arrow]"),
        _ => self.windows.input.flush_buffer()
    }
    Game::set_last_keypress(k);
    return k;
  }

  fn update_state(&mut self) {
    match Game::get_last_keypress() {
      Some(ks) => {
        match ks {
          Key { printable: '/', .. } => {
            let mut is: Box<AttackInputGameState> = Box::new(AttackInputGameState::new());
            is.weapon = "Heroic Sword".to_string();
            self.game_state = is as Box<GameState>;
          },
          Key { code: Number6, ..  } => {
            if ks.shift {
              let mut is: Box<AttackInputGameState> = Box::new(AttackInputGameState::new());
              is.weapon = "Boomerange".to_string();
              self.game_state = is as Box<GameState>;
            }
          },
          Key { code: Number8, ..  } => {
            if ks.shift {
              let mut is: Box<AttackInputGameState> = Box::new(AttackInputGameState::new());
              is.weapon = "Deadly Bomb".to_string();
              self.game_state = is as Box<GameState>;
            }
          },
          Key { code: Number5, ..  } => {
            if ks.shift {
              let mut is: Box<AttackInputGameState> = Box::new(AttackInputGameState::new());
              is.weapon = "Delicious Lettuce".to_string();
              self.game_state = is as Box<GameState>;
            }
          },
          Key { code: Shift, ..  } => {}
          _ => {
            let ms: Box<GameState> = Box::new(MovementGameState::new());
            self.game_state = ms;
          }
        }
      },
      _ => {}
    }
  }

  pub fn get_last_keypress() -> Option<Key> {
    unsafe { LAST_KEYPRESS }
  }

  pub fn set_last_keypress(key: Key) {
    unsafe { LAST_KEYPRESS = Some(key); }
  }

  pub fn get_character_point() -> Point {
    unsafe { CHAR_LOCATION }
  }

  pub fn set_character_point(point: Point) {
    unsafe { CHAR_LOCATION = point; }
  }
}

pub struct Windows {
  pub stats: Box<WindowComponent>,
  pub input: Box<WindowComponent>,
  pub messages: Box<WindowComponent>,
  pub map: Box<WindowComponent>,
}

impl Windows {
  fn all_windows(&mut self) -> Vec<&mut Box<WindowComponent>> {
    let windows = vec![
      &mut self.stats,
      &mut self.input,
      &mut self.messages,
      &mut self.map
    ];
    return windows;
  }
}

pub trait GameState {
  fn enter(&self, &mut Windows) {}
  fn exit(&self) {}

  fn should_update_state(&self) -> bool;

  fn update(&mut self, npcs: &mut Vec<Box<Actor>>, character: &mut Actor, windows: &mut Windows);

  fn render(&mut self, renderer: &mut Box<RenderingComponent>, npcs: &Vec<Box<Actor>>, character: &Actor, windows: &mut Windows) {
    renderer.before_render_new_frame();
    for window in windows.all_windows().iter_mut() {
      renderer.attach_window(*window);
    }
    for npc in npcs.iter() {
        npc.render(renderer);
    }
    character.render(renderer);
    renderer.after_render_new_frame();
  }
}

struct MovementGameState;
impl MovementGameState {
  fn new() -> MovementGameState {
    MovementGameState
  }
}

impl GameState for MovementGameState {
  fn should_update_state(&self) -> bool {
    true
  }

  fn update(&mut self, npcs: &mut Vec<Box<Actor>>, character: &mut Actor, windows: &mut Windows) {
    character.update();
    Game::set_character_point(character.position);
    for npc in npcs.iter_mut() {
        npc.update();
    }
  }
}

struct AttackInputGameState {
  should_update_state: bool,
  weapon: String
}
impl AttackInputGameState {
  fn new() -> AttackInputGameState {
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

  fn update(&mut self, npcs: &mut Vec<Box<Actor>>, character: &mut Actor, windows: &mut Windows) {
    match Game::get_last_keypress() {
      Some(ks) => {
        let mut msg = "You attack ".to_string();
        match ks {
          Key {code: Up, .. } => {
            msg.push_str("up");
            self.should_update_state = true;
          },
          Key {code: Down, .. } => {
            msg.push_str("down");
            self.should_update_state = true;
          },
          Key {code: Left, .. } => {
            msg.push_str("left");
            self.should_update_state = true;
          },
          Key {code: Right, .. } => {
            msg.push_str("right");
            self.should_update_state = true;
          },
          _ => {}
        }

        if self.should_update_state {
          msg.push_str(" with your ");
          msg.push_str(&self.weapon);
          msg.push_str("!");
          windows.messages.buffer_message(&msg);
        }
      },
      _ => {},
    }
  }
}





