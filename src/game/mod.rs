extern crate tcod;

use self::tcod::RootConsole;
use self::tcod::input::Key;
use util::{Bound, Point};
use rendering::{RenderingComponent, TcodRenderingComponent};
use traits::Updates;
use character::Character;

pub struct Game {
    pub exit: bool,
    pub window_bounds: Bound,
    pub rendering_component: Box<RenderingComponent>
}

impl Game {
  pub fn new() -> Game {
   let bound = Bound {
      min: Point { x: 0, y: 0 },
      max: Point { x: 79, y: 49}
    };
    let con = RootConsole::initializer().size(bound.max.x, bound.max.y).title("Roguelike!").init();

    let rc: Box<RenderingComponent> = Box::new(TcodRenderingComponent::new(con));
    Game {
      exit: false,
      window_bounds: bound,
      rendering_component: rc
    }
  }

  pub fn render(&mut self, npcs: &Vec<Box<Updates>>, c: Character) {
    self.rendering_component.before_render_new_frame();
    for i in npcs.iter() {
        i.render(&mut self.rendering_component);
    }
    c.render(&mut self.rendering_component);
    self.rendering_component.after_render_new_frame();
  }

  pub fn update(&mut self, npcs: &mut Vec<Box<Updates>>, c: &mut Character, keypress: Key) {
    c.update(keypress, self);
    for i in npcs.iter_mut() {
        i.update(&self);
    }
  }

  pub fn wait_for_keypress(&mut self) -> Key {
    self.rendering_component.wait_for_keypress()
  }
}
