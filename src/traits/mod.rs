extern crate tcod;
use self::tcod::{Console};
use game::Game;
use rendering::RenderingComponent;

pub trait Updates {
    fn update(&mut self, Game);
    fn render(&self, &mut Box<RenderingComponent>);
}