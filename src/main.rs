extern crate tcod;
extern crate roguelike;

use roguelike::game::Game;
use roguelike::actor::Actor;
use tcod::input::Key;
use tcod::input::KeyCode::{Escape};

fn main() {
    let mut game = Game::new();
    let mut c = Actor::heroine(40, 25, game.window_bounds);
    let mut npcs: Vec<Box<Actor>> = vec![
        Box::new(Actor::dog(10, 10, game.window_bounds)),
        Box::new(Actor::cat(40, 25, game.window_bounds))
    ];

    // render
    game.render(&npcs, &c);

    // our game loop
    while !(game.rendering_component.window_closed() || game.exit) {
        // wait for user input
        let keypress = game.wait_for_keypress();

        match keypress {
            Key {code: Escape, .. } => game.exit = true,
            _ => {}
        }
        game.update(&mut npcs, &mut c);

        game.render(&npcs, &c);
    }
}
