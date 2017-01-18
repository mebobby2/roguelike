extern crate tcod;

use self::tcod::{Console, RootConsole, BackgroundFlag};

use util::{Point};

pub struct TcodRenderingComponent {
    pub console: RootConsole
}

impl TcodRenderingComponent {
    pub fn before_render_new_frame(&mut self) {
        //self.console.clear();
    }

    pub fn render_object(&mut self, position: Point, symbol: char) {
        //self.console.put_char(position.x, position.y, symbol, BackgroundFlag::Set);
    }

    pub fn after_render_new_frame(&mut self) {
        //self.console.flush();
    }
}