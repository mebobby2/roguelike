extern crate tcod;

use self::tcod::{Console, RootConsole, TextAlignment};
use self::tcod::input::Key;

use util::{Point, Bound};
use input::{TcodInputComponent, InputComponent, KeyboardInput};
use rendering::windows::WindowComponent;

#[derive(Copy, Clone)]
pub enum Color {
    Red,
    Blue,
    Black,
    White
}

pub trait RenderingComponent {
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, Point, char, Color, Color);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> KeyboardInput;
    fn window_closed(&mut self) -> bool;
    fn attach_window(&mut self, window: &mut Box<WindowComponent>);
    fn translate_color(&self, Color) -> tcod::Color;
}

pub struct TcodRenderingComponent {
    console: RootConsole,
    input_component: Box<InputComponent<Key>>
}

impl TcodRenderingComponent {
    pub fn new(bounds: Bound) -> TcodRenderingComponent {
        let console = RootConsole::initializer().size(bounds.max.x + 1, bounds.max.y + 1).title("Roguelike!").init();

        let ic: Box<InputComponent<Key>> = Box::new(TcodInputComponent::new());

        TcodRenderingComponent {
            console: console,
            input_component: ic
        }
    }
}

impl RenderingComponent for TcodRenderingComponent {
    fn before_render_new_frame(&mut self) {
        self.console.clear();
    }

    fn render_object(&mut self, position: Point, symbol: char, foreground: Color, background: Color) {
        let f = self.translate_color(foreground);
        let b = self.translate_color(background);
        self.console.put_char_ex(position.x, position.y, symbol, f, b);
    }

    fn after_render_new_frame(&mut self) {
        self.console.flush();
    }

    fn wait_for_keypress(&mut self) -> KeyboardInput {
        let k = self.console.wait_for_keypress(true);
        self.input_component.translate_input(k)
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

    fn translate_color(&self, input: Color) -> tcod::Color {
        match input {
            Color::Red   => tcod::Color::new(255u8, 0u8, 0u8),
            Color::Blue  => tcod::Color::new(0u8, 0u8, 255u8),
            Color::White => tcod::Color::new(255u8, 255u8, 255u8),
            Color::Black => tcod::Color::new(0u8, 0u8, 0u8)
        }
    }
}
