extern crate tcod;
extern crate roguelike;

use roguelike::game::Game;
use roguelike::actor::Actor;
use roguelike::input::GameKeyCode;
use roguelike::input::GameKey::{SpecialKey};

fn main() {
    let mut game = Game::new();
    let game.maps.friends.push_actor(Point::new(10, 10), Box::new(Actor::dog(10, 10, game.windows.get_map_bounds())));
    let game.maps.friends.push_actor(Point::new(40, 25), Box::new(Actor::cat(40, 25, game.windows.get_map_bounds())));
    let game.maps.enemies.push_actor(Point::new(20, 20), Box::new(Actor::dog(20, 20, game.windows.get_map_bounds())));
    let game.maps.pcs.push_actor(Game::get_character_point(), Box::new(Actor::heroine(game.windows.get_map_bounds())));

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
