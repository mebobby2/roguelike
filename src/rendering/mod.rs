extern crate tcod;

use self::tcod::{Console, RootConsole, BackgroundFlag};
use self::tcod::input::Key;

use util::{Point};

pub trait RenderingComponent {
    //fn new(RootConsole) -> Self;
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, Point, char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> Key;
    fn window_closed(&mut self) -> bool;
}

pub struct TcodRenderingComponent {
    pub console: RootConsole
}

impl RenderingComponent for TcodRenderingComponent {
    // fn new(console: RootConsole) -> TcodRenderingComponent {
    //     TcodRenderingComponent {
    //         console: console
    //     }
    // }

    fn before_render_new_frame(&mut self) {
        self.console.clear();
    }

    fn render_object(&mut self, position: Point, symbol: char) {
        self.console.put_char(position.x, position.y, symbol, BackgroundFlag::Set);
    }

    fn after_render_new_frame(&mut self) {
        self.console.flush();
    }

    fn wait_for_keypress(&mut self) -> Key {
        self.console.wait_for_keypress(true)
    }

    fn window_closed(&mut self) -> bool {
        self.console.window_closed()
    }
}