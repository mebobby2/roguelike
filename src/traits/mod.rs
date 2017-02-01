extern crate tcod;
use self::tcod::{Console};
use rendering::RenderingComponent;

pub trait Updates {
    fn update(&mut self);
    fn render(&self, &mut Box<RenderingComponent>);
}