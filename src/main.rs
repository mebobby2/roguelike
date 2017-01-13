extern crate tcod;
extern crate rand;

use tcod::{Console, RootConsole, BackgroundFlag};
use tcod::input::Key;
use tcod::input::{KeyCode};
use tcod::input::KeyCode::{Up, Down, Left, Right, Escape, Enter};
use rand::Rng;

#[derive(Copy, Clone)]
struct Point {
  x: i32,
  y: i32
}

fn render(con: &mut RootConsole, c_point: Point, d_point: Point) {
    con.clear();
    con.put_char(c_point.x, c_point.y, '@', BackgroundFlag::Set);
    con.put_char(d_point.x, d_point.y, 'd', BackgroundFlag::Set);
    con.flush();
}

fn main() {
    let conX = 80i32;
    let conY = 50i32;
    let mut char_point = Point { x: 40, y: 25 };
    let mut dog_point = Point { x: 10, y: 10 };

    let mut con = RootConsole::initializer().size(conX, conY).title("libtcod Rust tutorial").init();
    let mut exit = false;

    // render
    render(&mut con, char_point, dog_point);

    // our game loop
    while !(con.window_closed() || exit) {
        // wait for user input
        let keypress = con.wait_for_keypress(true);

        // update game state
        match keypress {
            Key {code: Escape, .. } => exit = true,
            Key {code: Up, .. } => {
                if char_point.y >= 1 {
                    char_point.y -= 1;
                }
            },
            Key {code: Down, .. } => {
                if char_point.y < (conY - 1) {
                    char_point.y += 1;
                }
            },
            Key {code: Left, .. } => {
                if char_point.x >= 1 {
                    char_point.x -= 1
                }
            },
            Key {code: Right, .. } => {
                if char_point.x < (conX -1) {
                    char_point.x += 1
                }
            },
            _ => {}
        }

        let offset_x = rand::thread_rng().gen_range(0, 3i32) - 1;
        if (dog_point.x + offset_x) > 0 && (dog_point.x + offset_x) < (conX - 1) {
            dog_point.x += offset_x;
        }

        let offset_y = rand::thread_rng().gen_range(0, 3i32) - 1;
        if (dog_point.y + offset_y) > 0 && (dog_point.y + offset_y) < (conY - 1) {
            dog_point.y += offset_y;
        }

        //render
        render(&mut con, char_point, dog_point);
    }
}
