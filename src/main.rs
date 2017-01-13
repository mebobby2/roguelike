extern crate tcod;
extern crate rand;

use tcod::{Console, RootConsole, BackgroundFlag};
use tcod::input::Key;
use tcod::input::KeyCode::{Up, Down, Left, Right, Escape};
use rand::Rng;

enum Contains {
    DoesContain,
    DoesNotContain
}

#[derive(Copy, Clone)]
struct Point {
  x: i32,
  y: i32
}

impl Point {
    fn offset_x(&self, offset: i32) -> Point {
        Point { x: self.x + offset, .. *self }
    }

    fn offset_y(&self, offset: i32) -> Point {
        Point { y: self.y + offset, .. *self }
    }

    fn offset(&self, offset: Point) -> Point {
        Point { x: self.x + offset.x, y: self.y + offset.y }
    }
}

#[derive(Copy, Clone)]
struct Bound {
    min: Point,
    max: Point
}

impl Bound {
    fn contains(&self, point: Point) -> Contains {
        if
            point.x >= self.min.x &&
            point.x < self.max.x &&
            point.y >= self.min.y &&
            point.y < self.max.y
        {
            Contains::DoesContain
        } else {
            Contains::DoesNotContain
        }
    }
}

#[derive(Copy, Clone)]
struct Game {
    exit: bool,
    window_bounds: Bound
}

#[derive(Copy, Clone)]
struct Character {
    position: Point,
    display_char: char
}

impl Character {
    fn new(x: i32, y: i32, dc: char) -> Character {
        Character { position: Point {x: x, y: y}, display_char: dc }
    }
}

struct NPC {
    position: Point,
    display_char: char
}

impl NPC {
    fn new(x: i32, y: i32, dc: char) -> NPC {
        NPC { position: Point { x: x, y: y }, display_char: dc }
    }
}

trait Updates {
    fn update(&mut self, tcod::input::Key, Game);
    fn render(&self, &mut Console);
}

impl Updates for Character {
    fn update(&mut self, keypress: tcod::input::Key, game: Game) {
        let mut offset = Point { x: 0, y: 0 };
        match keypress {
            Key {code: Up, .. } => {
               offset.y = -1;
            },
            Key {code: Down, .. } => {
                offset.y = 1;
            },
            Key {code: Left, .. } => {
                offset.x = -1;
            },
            Key {code: Right, .. } => {
                offset.x = 1;
            },
            _ => {}
        }

        match game.window_bounds.contains(self.position.offset(offset)) {
            Contains::DoesContain => self.position = self.position.offset(offset),
            Contains::DoesNotContain => {},
        }
    }

    fn render(&self, console: &mut Console) {
        console.put_char(self.position.x, self.position.y, self.display_char, BackgroundFlag::Set);
    }
}

impl Updates for NPC {
    fn update(&mut self, keypress: tcod::input::Key, game: Game) {
        let offset_x = rand::thread_rng().gen_range(0, 3i32) - 1;
        match game.window_bounds.contains(self.position.offset_x(offset_x)) {
            Contains::DoesContain => self.position = self.position.offset_x(offset_x),
            Contains::DoesNotContain => {},
        }

        let offset_y = rand::thread_rng().gen_range(0, 3i32) - 1;
        match game.window_bounds.contains(self.position.offset_y(offset_y)) {
            Contains::DoesContain => self.position = self.position.offset_y(offset_y),
            Contains::DoesNotContain => {},
        }
    }

    fn render(&self, console: &mut Console) {
        console.put_char(self.position.x, self.position.y, self.display_char, BackgroundFlag::Set);
    }
}

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

    let mut exit = false;

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
