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
    console: RootConsole
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
        let mut line = 0i32;
        let bounds = window.get_bounds();
        let messages = window.get_messages();

        for message in messages.iter() {
            window.print_message(0, line, TextAlignment::Left, &message[..]);
            line = line + 1;
        }

        let console = window.get_console();

        tcod::console::blit(&*console,
                            (0, 0),
                            (bounds.max.x + 1, bounds.max.y + 1),
                            &mut self.console,
                            (bounds.min.x, bounds.min.y),
                            1f32,
                            1f32);
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

    fn buffer_message(&mut self, text: &str) {
        let max = self.get_max_messages();
        let message = String::from(text);
        let messages = self.get_mut_messages();

        messages.insert(0, Box::new(message));
        messages.truncate(max as usize);
    }

    fn flush_buffer(&mut self) {
        let max = self.get_max_messages();
        let messages = self.get_mut_messages();

        for _ in 0..max {
            messages.insert(0, Box::new(String::from("")));
        }
        messages.truncate(max as usize);
    }

    fn get_mut_messages(&mut self) -> &mut Vec<Box<String>>;

    fn get_messages(&self) -> Vec<Box<String>>;

    fn get_max_messages(&self) -> u32;
}

pub struct TcodStatsWindowComponent {
    console: OffscreenConsole,
    background_color: Color,
    bounds: Bound,
    messages: Vec<Box<String>>,
    max_messages: u32
}

pub struct TcodInputWindowComponent {
    console: OffscreenConsole,
    background_color: Color,
    bounds: Bound,
    messages: Vec<Box<String>>,
    max_messages: u32
}

pub struct TcodMessagesWindowComponent {
    console: OffscreenConsole,
    background_color: Color,
    bounds: Bound,
    messages: Vec<Box<String>>,
    max_messages: u32
}

pub struct TcodMapWindowComponent {
    console: OffscreenConsole,
    background_color: Color,
    bounds: Bound,
    messages: Vec<Box<String>>,
    max_messages: u32
}

impl TcodStatsWindowComponent {
    pub fn new(bounds: Bound) -> TcodStatsWindowComponent {
        let height = bounds.max.y - bounds.min.y + 1;
        let width = bounds.max.x - bounds.min.x + 1;
        let console = OffscreenConsole::new(width, height);

        let red = Color::new(0u8, 0u8, 0u8);
        TcodStatsWindowComponent {
            console: console,
            background_color: red,
            bounds: bounds,
            messages: vec![],
            max_messages: 10u32
        }
    }
}

impl TcodInputWindowComponent {
    pub fn new(bounds: Bound) -> TcodStatsWindowComponent {
        let height = bounds.max.y - bounds.min.y + 1;
        let width = bounds.max.x - bounds.min.x + 1;
        let console = OffscreenConsole::new(width, height);

        let red = Color::new(0u8, 0u8, 0u8);
        TcodStatsWindowComponent {
            console: console,
            background_color: red,
            bounds: bounds,
            messages: vec![],
            max_messages: 2u32
        }
    }
}

impl TcodMessagesWindowComponent {
    pub fn new(bounds: Bound) -> TcodStatsWindowComponent {
        let height = bounds.max.y - bounds.min.y + 1;
        let width = bounds.max.x - bounds.min.x + 1;
        let console = OffscreenConsole::new(width, height);

        let red = Color::new(0u8, 0u8, 0u8);
        TcodStatsWindowComponent {
            console: console,
            background_color: red,
            bounds: bounds,
            messages: vec![],
            max_messages: 10u32
        }
    }
}

impl TcodMapWindowComponent {
    pub fn new(bounds: Bound) -> TcodStatsWindowComponent {
        let height = bounds.max.y - bounds.min.y + 1;
        let width = bounds.max.x - bounds.min.x + 1;
        let console = OffscreenConsole::new(width, height);

        let red = Color::new(0u8, 0u8, 0u8);
        TcodStatsWindowComponent {
            console: console,
            background_color: red,
            bounds: bounds,
            messages: vec![],
            max_messages: 10u32
        }
    }
}

impl WindowComponent for TcodStatsWindowComponent {
    fn get_console(&mut self) -> &mut OffscreenConsole { &mut self.console }
    fn get_bounds(&self) -> Bound { self.bounds }
    fn get_bg_color(&self) -> Color { self.background_color }

    fn get_mut_messages(&mut self) -> &mut Vec<Box<String>> {
        &mut self.messages
    }

    fn get_messages(&self) -> Vec<Box<String>> {
        self.messages.clone()
    }

    fn get_max_messages(&self) -> u32 {
        self.max_messages
    }
}

impl WindowComponent for TcodInputWindowComponent {
    fn get_console(&mut self) -> &mut OffscreenConsole { &mut self.console }
    fn get_bounds(&self) -> Bound { self.bounds }
    fn get_bg_color(&self) -> Color { self.background_color }

    fn get_mut_messages(&mut self) -> &mut Vec<Box<String>> {
        &mut self.messages
    }

    fn get_messages(&self) -> Vec<Box<String>> {
        self.messages.clone()
    }

    fn get_max_messages(&self) -> u32 {
        self.max_messages
    }
}

impl WindowComponent for TcodMessagesWindowComponent {
    fn get_console(&mut self) -> &mut OffscreenConsole { &mut self.console }
    fn get_bounds(&self) -> Bound { self.bounds }
    fn get_bg_color(&self) -> Color { self.background_color }

    fn get_mut_messages(&mut self) -> &mut Vec<Box<String>> {
        &mut self.messages
    }

    fn get_messages(&self) -> Vec<Box<String>> {
        self.messages.clone()
    }

    fn get_max_messages(&self) -> u32 {
        self.max_messages
    }
}

impl WindowComponent for TcodMapWindowComponent {
    fn get_console(&mut self) -> &mut OffscreenConsole { &mut self.console }
    fn get_bounds(&self) -> Bound { self.bounds }
    fn get_bg_color(&self) -> Color { self.background_color }

    fn get_mut_messages(&mut self) -> &mut Vec<Box<String>> {
        &mut self.messages
    }

    fn get_messages(&self) -> Vec<Box<String>> {
        self.messages.clone()
    }

    fn get_max_messages(&self) -> u32 {
        self.max_messages
    }
}