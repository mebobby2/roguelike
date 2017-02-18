extern crate tcod;
extern crate roguelike;

use roguelike::game::Game;
use roguelike::actor::Actor;
use roguelike::input::GameKeyCode;
use roguelike::input::GameKey::{SpecialKey};

fn main() {
    let mut game = Game::new();
    let mut c = Actor::heroine(game.windows.map.get_bounds());
    let mut npcs: Vec<Box<Actor>> = vec![
        Box::new(Actor::dog(10, 10, game.windows.map.get_bounds())),
        Box::new(Actor::cat(40, 25, game.windows.map.get_bounds())),
        Box::new(Actor::kobold(20, 20, game.windows.map.get_bounds()))
    ];

    // render
    game.render(&npcs, &c);

    // our game loop
    while !(game.rendering_component.window_closed() || game.exit) {
        // wait for user input
        let keypress = game.wait_for_keypress();

        match keypress.key {
            SpecialKey(GameKeyCode::Escape) => game.exit = true,
            _ => {}
        }
        game.update(&mut npcs, &mut c);

        game.render(&npcs, &c);
    }
}
