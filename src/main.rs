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

fn render(con: &mut RootConsole, objs: &Vec<Box<Updates>>) {
    con.clear();
    for i in objs.iter() {
        i.render(con);
    }
    con.flush();
}

fn update(objs: &mut Vec<Box<Updates>>, keypress: tcod::input::Key, game: Game) {
    for i in objs.iter_mut() {
        i.update(keypress, game);
    }
}

fn main() {
    let mut game = Game { exit: false, window_bounds: Bound { min: Point { x: 0, y: 0}, max: Point { x: 79, y: 49 } }};
    let mut con = RootConsole::initializer().size(game.window_bounds.max.x, game.window_bounds.max.y).title("libtcod Rust tutorial").init();

    let c = Box::new(Character::new(40, 25, '@')); //box Character::new(40, 25, '@') as Box<Updates>;
    let d = Box::new(NPC::new(10, 10, 'd')); //box NPC::new(10, 10, 'd') as Box<Updates>;
    let mut objs: Vec<Box<Updates>> = vec![c,d];

    // render
    render(&mut con, &objs);

    // our game loop
    while !(con.window_closed() || game.exit) {
        // wait for user input
        let keypress = con.wait_for_keypress(true);

        match keypress {
            Key {code: Escape, .. } => game.exit = true,
            _ => {}
        }
        update(&mut objs, keypress, game);

        render(&mut con, &objs);
    }
}
