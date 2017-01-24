extern crate tcod;
extern crate rand;
extern crate roguelike;

use tcod::RootConsole;
use tcod::input::Key;
use tcod::input::KeyCode::{Escape};
use roguelike::util::{Point, Bound};
use roguelike::game::Game;
use roguelike::traits::Updates;
use roguelike::character::Character;
use roguelike::npc::NPC;
use roguelike::rendering::{RenderingComponent,TcodRenderingComponent};

fn render(rendering_component: &mut Box<RenderingComponent>, npcs: &Vec<Box<Updates>>, c: Character) {
    rendering_component.before_render_new_frame();
    for i in npcs.iter() {
        i.render(rendering_component);
    }
    c.render(rendering_component);
    rendering_component.after_render_new_frame();
}

fn update(npcs: &mut Vec<Box<Updates>>, c: &mut Character, keypress: tcod::input::Key, game: Game) {
    c.update(keypress, game);
    for i in npcs.iter_mut() {
        i.update(game);
    }
}

fn main() {
    let mut game = Game { exit: false, window_bounds: Bound { min: Point { x: 0, y: 0}, max: Point { x: 79, y: 49 } }};
    let con = RootConsole::initializer().size(game.window_bounds.max.x, game.window_bounds.max.y).title("libtcod Rust tutorial").init();

    let mut rendering_component: Box<RenderingComponent> = Box::new(TcodRenderingComponent::new(con));

    let mut c = Character::new(40, 25, '@');
    let d = Box::new(NPC::new(10, 10, 'd'));
    let ct = Box::new(NPC::new(40, 25, 'c'));

    let mut npcs: Vec<Box<Updates>> = vec![d,ct];

    // render
    render(&mut rendering_component, &npcs, c);

    // our game loop
    while !(rendering_component.window_closed() || game.exit) {
        // wait for user input
        let keypress = rendering_component.wait_for_keypress();

        match keypress {
            Key {code: Escape, .. } => game.exit = true,
            _ => {}
        }
        update(&mut npcs, &mut c, keypress, game);

        render(&mut rendering_component, &npcs, c);
    }
}
