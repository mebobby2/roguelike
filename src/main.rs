extern crate tcod;
extern crate roguelike;

use roguelike::game::Game;
use roguelike::actor::Actor;
use roguelike::movement::{RandomMovementComponent, MovementComponent, TcodUserMovementComponent};
use tcod::input::Key;
use tcod::input::KeyCode::{Escape};

fn main() {
    let mut game = Game::new();
    let char_mc: Box<MovementComponent> = Box::new(TcodUserMovementComponent::new(game.window_bounds));
    let mut c = Actor::new(40, 25, '@', char_mc);

    let cmc: Box<MovementComponent> = Box::new(RandomMovementComponent::new(game.window_bounds));
    let dmc: Box<MovementComponent> = Box::new(RandomMovementComponent::new(game.window_bounds));
    let mut npcs: Vec<Box<Actor>> = vec![
        Box::new(Actor::new(10, 10, 'd', dmc)),
        Box::new(Actor::new(40, 25, 'c', cmc))
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
