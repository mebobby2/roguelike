extern crate tcod;
use self::tcod::{Console};
use game::Game;

pub trait Updates {
    fn update(&mut self, tcod::input::Key, Game);
    fn render(&self, &mut Console);
}