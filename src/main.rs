extern crate tcod;
extern crate roguelike;

use roguelike::game::Game;
use roguelike::input::GameKeyCode;
use roguelike::input::GameKey::{SpecialKey};

fn main() {
    let mut game = Game::new();

    // render
    game.render();

    // our game loop
    while !(game.rendering_component.window_closed() || game.exit) {
        // wait for user input
        let keypress = game.wait_for_keypress();

        match keypress.key {
            SpecialKey(GameKeyCode::Escape) => game.exit = true,
            _ => {}
        }
        game.update();

        game.render();
    }
}
