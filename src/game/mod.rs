extern crate tcod;

use self::tcod::input::Key;
use util::{Bound, Point};
use rendering::{RenderingComponent, TcodRenderingComponent, WindowComponent, TcodStatsWindowComponent};
use actor::Actor;

pub struct Game {
    pub exit: bool,
    pub window_bounds: Bound,
    pub rendering_component: Box<RenderingComponent>,
    pub stats_window: Box<WindowComponent>
}

static mut LAST_KEYPRESS: Option<Key> = None;
static mut CHAR_LOCATION: Point = Point { x: 40, y: 25 };

impl Game {
  pub fn new() -> Game {
    let total_bounds = Bound::new(0, 0, 99, 61);
    let stats_bounds = Bound::new(79, 0, 99, 49);

    let rc: Box<RenderingComponent> = Box::new(TcodRenderingComponent::new(total_bounds));
    let sw: Box<WindowComponent> = Box::new(TcodStatsWindowComponent::new(stats_bounds));
    Game {
      exit: false,
      window_bounds: total_bounds,
      rendering_component: rc,
      stats_window: sw
    }
  }

  pub fn render(&mut self, npcs: &Vec<Box<Actor>>, c: &Actor) {
    self.rendering_component.before_render_new_frame();
    self.rendering_component.attach_window(&mut self.stats_window);
    for i in npcs.iter() {
        i.render(&mut self.rendering_component);
    }
    c.render(&mut self.rendering_component);
    self.rendering_component.after_render_new_frame();
  }

  pub fn update(&mut self, npcs: &mut Vec<Box<Actor>>, c: &mut Actor) {
    c.update();
    Game::set_character_point(c.position);
    for i in npcs.iter_mut() {
        i.update();
    }
  }

  pub fn wait_for_keypress(&mut self) -> Key {
    let k = self.rendering_component.wait_for_keypress();
    Game::set_last_keypress(k);
    return k;
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
