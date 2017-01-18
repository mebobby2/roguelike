extern crate tcod;
extern crate rand;
extern crate roguelike;

use tcod::{Console, RootConsole};
use tcod::input::Key;
use tcod::input::KeyCode::{Escape};
use roguelike::util::{Point, Bound};
use roguelike::game::Game;
use roguelike::traits::Updates;
use roguelike::character::Character;
use roguelike::npc::NPC;

fn render(con: &mut RootConsole, npcs: &Vec<Box<Updates>>, c: Character) {
    con.clear();
    for i in npcs.iter() {
        i.render(con);
    }
    c.render(con);
    con.flush();
}

fn update(npcs: &mut Vec<Box<Updates>>, c: &mut Character, keypress: tcod::input::Key, game: Game) {
    c.update(keypress, game);
    for i in npcs.iter_mut() {
        i.update(keypress, game);
    }
}

fn main() {
    let mut game = Game { exit: false, window_bounds: Bound { min: Point { x: 0, y: 0}, max: Point { x: 79, y: 49 } }};
    let mut con = RootConsole::initializer().size(game.window_bounds.max.x, game.window_bounds.max.y).title("libtcod Rust tutorial").init();

    let mut c = Character::new(40, 25, '@');
    let d = Box::new(NPC::new(10, 10, 'd'));
    let ct = Box::new(NPC::new(40, 25, 'c'));

    let mut npcs: Vec<Box<Updates>> = vec![d,ct];

    // render
    render(&mut con, &npcs, c);

    // our game loop
    while !(con.window_closed() || game.exit) {
        // wait for user input
        let keypress = con.wait_for_keypress(true);

        match keypress {
            Key {code: Escape, .. } => game.exit = true,
            _ => {}
        }
        update(&mut npcs, &mut c, keypress, game);

        render(&mut con, &npcs, c);
    }
}
