extern crate tcod;
extern crate roguelike;

use tcod::input::Key;
use tcod::input::KeyCode::{Escape};
use roguelike::game::Game;
use roguelike::traits::Updates;
use roguelike::character::Character;
use roguelike::npc::NPC;
use roguelike::movement::{RandomMovementComponent, MovementComponent};

fn main() {
    let mut game = Game::new();
    let mut c = Character::new(40, 25, '@');

    let cmc: Box<MovementComponent> = Box::new(RandomMovementComponent::new(game.window_bounds));
    let dmc: Box<MovementComponent> = Box::new(RandomMovementComponent::new(game.window_bounds));
    let mut npcs: Vec<Box<Updates>> = vec![
        Box::new(NPC::new(10, 10, 'd', dmc)),
        Box::new(NPC::new(40, 25, 'c', cmc))
    ];

    // render
    game.render(&npcs, c);

    // our game loop
    while !(game.rendering_component.window_closed() || game.exit) {
        // wait for user input
        let keypress = game.wait_for_keypress();

        match keypress {
            Key {code: Escape, .. } => game.exit = true,
            _ => {}
        }
        game.update(&mut npcs, &mut c, keypress);

        game.render(&npcs, c);
    }
}
