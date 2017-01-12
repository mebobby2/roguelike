extern crate tcod;

use tcod::{Console, RootConsole, BackgroundFlag};
use tcod::input::Key;
use tcod::input::{KeyCode};
use tcod::input::KeyCode::{Up, Down, Left, Right, Escape, Enter};

fn render(con: &mut RootConsole) {
    con.clear();
    con.put_char(40, 25, '@', BackgroundFlag::Set);
    con.flush();
}

fn main() {
    let mut con = RootConsole::initializer().size(80, 50).title("libtcod Rust tutorial").init();
    let mut exit = false;

    // render
    render(&mut con);

    // our game loop
    while !(con.window_closed() || exit) {
        // wait for user input
        let keypress = con.wait_for_keypress(true);

        // update game state
        match keypress {
            Key {code: Escape, .. } => exit = true,
            _ => {},
        }

        //render
        render(&mut con);
    }
}
