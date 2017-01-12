extern crate tcod;

use tcod::{Console, RootConsole, BackgroundFlag};
use tcod::input::Key;
use tcod::input::{KeyCode};
use tcod::input::KeyCode::{Up, Down, Left, Right, Escape, Enter};

fn render(con: &mut RootConsole, x: i32, y: i32) {
    con.clear();
    con.put_char(x, y, '@', BackgroundFlag::Set);
    con.flush();
}

fn main() {
    let conX = 80i32;
    let conY = 50i32;
    let mut charX = 40i32;
    let mut charY = 25i32;

    let mut con = RootConsole::initializer().size(conX, conY).title("libtcod Rust tutorial").init();
    let mut exit = false;

    // render
    render(&mut con, charX, charY);

    // our game loop
    while !(con.window_closed() || exit) {
        // wait for user input
        let keypress = con.wait_for_keypress(true);

        // update game state
        match keypress {
            Key {code: Escape, .. } => exit = true,
            Key {code: Up, .. } => {
                if charY >= 1 {
                    charY -= 1;
                }
            },
            Key {code: Down, .. } => {
                if charY < (conY - 1) {
                    charY += 1;
                }
            },
            _ => {}
        }

        //render
        render(&mut con, charX, charY);
    }
}
