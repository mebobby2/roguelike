extern crate tcod;

use self::tcod::{Console, RootConsole, BackgroundFlag, Color, OffscreenConsole, TextAlignment};
use self::tcod::input::Key;

use util::{Point, Bound};

pub trait RenderingComponent {
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, Point, char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> Key;
    fn window_closed(&mut self) -> bool;
    fn attach_window(&mut self, window: &mut Box<WindowComponent>);
}

pub struct TcodRenderingComponent {
    pub console: RootConsole
}

impl TcodRenderingComponent {
    pub fn new(bounds: Bound) -> TcodRenderingComponent {
        let console = RootConsole::initializer().size(bounds.max.x + 1, bounds.max.y + 1).title("Roguelike!").init();

        TcodRenderingComponent {
            console: console
        }
    }
}

impl RenderingComponent for TcodRenderingComponent {
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

    fn attach_window(&mut self, window: &mut Box<WindowComponent>) {
        window.clear();
        window.print_message(0, 0, TextAlignment::Left, "Sup foo!");
        window.print_message(0, 1, TextAlignment::Left, "Nothing fool!");
        let bounds = window.get_bounds();
        let console = window.get_console();

        tcod::console::blit(&*console,
                            (0, 0),
                            (bounds.max.x + 1, bounds.max.y + 1),
                            &mut self.console,
                            (bounds.min.x, bounds.min.y),
                            1.0,
                            1.0);

        // tcod::console::blit(con, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT),
        //                 root, (0, 0),
        //                 1.0, 1.0);
    }
}

pub trait WindowComponent {
    fn get_bounds(&self) -> Bound;
    fn get_bg_color(&self) -> Color;
    fn get_console(&mut self) -> &mut OffscreenConsole;

    fn clear(&mut self) {
        let color = self.get_bg_color();
        let mut console = self.get_console();
        console.set_default_background(color);
        console.clear();
    }

    fn print_message(&mut self, x: i32, y: i32, alignment: TextAlignment, text: &str) {
        let mut console = self.get_console();
        console.print_ex(x, y, BackgroundFlag::Set, alignment, text);
    }
}

pub struct TcodStatsWindowComponent {
    pub console: OffscreenConsole,
    pub background_color: Color,
    bounds: Bound
}

impl TcodStatsWindowComponent {
    pub fn new(bounds: Bound) -> TcodStatsWindowComponent {
        let height = bounds.max.y - bounds.min.y + 1;
        let width = bounds.max.x - bounds.min.x + 1;
        let console = OffscreenConsole::new(width, height);

        let red = Color::new(255u8, 0u8, 0u8);
        TcodStatsWindowComponent {
            console: console,
            background_color: red,
            bounds: bounds
        }
    }
}

impl WindowComponent for TcodStatsWindowComponent {
    fn get_console(&mut self) -> &mut OffscreenConsole { &mut self.console }
    fn get_bounds(&self) -> Bound { self.bounds }
    fn get_bg_color(&self) -> Color { self.background_color }
}