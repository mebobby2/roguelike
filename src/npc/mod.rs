extern crate tcod;
extern crate rand;

use self::tcod::{Console, BackgroundFlag};
use self::rand::Rng;

use traits::Updates;
use util::{Point, Contains};
use game::Game;

pub struct NPC {
    position: Point,
    display_char: char
}

impl NPC {
    pub fn new(x: i32, y: i32, dc: char) -> NPC {
        NPC { position: Point { x: x, y: y }, display_char: dc }
    }
}

impl Updates for NPC {
    fn update(&mut self, game: Game) {
        let offset_x = rand::thread_rng().gen_range(0, 3i32) - 1;
        match game.window_bounds.contains(self.position.offset_x(offset_x)) {
            Contains::DoesContain => self.position = self.position.offset_x(offset_x),
            Contains::DoesNotContain => {},
        }

        let offset_y = rand::thread_rng().gen_range(0, 3i32) - 1;
        match game.window_bounds.contains(self.position.offset_y(offset_y)) {
            Contains::DoesContain => self.position = self.position.offset_y(offset_y),
            Contains::DoesNotContain => {},
        }
    }

    fn render(&self, console: &mut Console) {
        console.put_char(self.position.x, self.position.y, self.display_char, BackgroundFlag::Set);
    }
}