extern crate tcod;

use self::tcod::{Console, BackgroundFlag, Color, OffscreenConsole, TextAlignment};

use util::Bound;

macro_rules! window_component_getters {
    () => {
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
    };
}

macro_rules! window_component_def {
    ($name:ident) => {
        pub struct $name {
            console: OffscreenConsole,
            background_color: Color,
            bounds: Bound,
            messages: Vec<Box<String>>,
            max_messages: u32
        }
    };
}

macro_rules! window_component_init {
    ($name:ident, $color:expr, $max_messages:expr) => {
        pub fn new(bounds: Bound) -> $name {
            let height = bounds.max.y - bounds.min.y + 1;
            let width = bounds.max.x - bounds.min.x + 1;
            let console = OffscreenConsole::new(width, height);

            $name {
                console: console,
                background_color: $color,
                bounds: bounds,
                messages: vec![],
                max_messages: $max_messages
            }
        }
    };
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

window_component_def!(TcodStatsWindowComponent);
impl TcodStatsWindowComponent {
    window_component_init!(TcodStatsWindowComponent, Color::new(0u8, 0u8, 0u8), 10u32);
}
impl WindowComponent for TcodStatsWindowComponent {
    window_component_getters!();
}

window_component_def!(TcodInputWindowComponent);
impl TcodInputWindowComponent {
    window_component_init!(TcodInputWindowComponent, Color::new(0u8, 0u8, 0u8), 2u32);
}
impl WindowComponent for TcodInputWindowComponent {
    window_component_getters!();
}

window_component_def!(TcodMessagesWindowComponent);
impl TcodMessagesWindowComponent {
    window_component_init!(TcodMessagesWindowComponent, Color::new(0u8, 0u8, 0u8), 9u32);
}
impl WindowComponent for TcodMessagesWindowComponent {
    window_component_getters!();
}

window_component_def!(TcodMapWindowComponent);
impl TcodMapWindowComponent {
    window_component_init!(TcodMapWindowComponent, Color::new(0u8, 0u8, 0u8), 10u32);
}
impl WindowComponent for TcodMapWindowComponent {
    window_component_getters!();
}

pub struct Windows {
  pub stats: Box<WindowComponent>,
  pub input: Box<WindowComponent>,
  pub messages: Box<WindowComponent>,
  pub map: Box<WindowComponent>,
}

impl Windows {
  pub fn all_windows(&mut self) -> Vec<&mut Box<WindowComponent>> {
    let windows = vec![
      &mut self.stats,
      &mut self.input,
      &mut self.messages,
      &mut self.map
    ];
    return windows;
  }
}
