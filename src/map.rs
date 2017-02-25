use rendering::windows::Windows;
use rendering::renderers::RenderingComponent;
use util::{Point, Bound};
use actor::Actor;

pub struct Maps {
  pub terrain: Box<Map>,
  pub enemies: Box<Map>,
  pub friends: Box<Map>,
  pub pcs: Box<Map>
}

impl Maps {
  pub fn new(size: Bound) -> Maps {
    let terrain = Box::new(Map::new(size));
    let enemies = Box::new(Map::new(size));
    let friends = Box::new(Map::new(size));
    let pcs = Box::new(Map::new(size));

    Maps {
      friends: friends,
      enemies: enemies,
      terrain: terrain,
      pcs: pcs
    }
  }

  pub fn update(&mut self, windows: &mut Windows) {
    self.pcs.update(windows);
    self.terrain.update(windows);
    self.friends.update(windows);
    self.pcs.update(windows);
  }

  pub fn render(&mut self, renderer: &mut Box<RenderingComponent>) {
    self.terrain.renderer(renderer);
    self.friends.renderer(renderer);
    self.enemies.renderer(renderer);
    self.pcs.renderer(renderer);
  }

  pub fn enemy_at(&self, point: Point) -> Option<&Box<Actor>> {
    let enemies_at_point = &self.enemies.content[point.x][point.y];
    if enemies_at_point.len() > 0 {
      Some(&enemies_at_point[0])
    } else {
      None
    }
  }
}

pub struct Map {
  pub content: Vec<Vec<Vec<Box<Actor>>>>,
  pub size: Bound
}

impl Map {
  pub fn new(size: Bound) -> Map {
    let content = Map::init_contents(size);
    Map {
      content: content,
      size: size
    }
  }

  fn init_contents(size: Bound) -> Vec<Vec<Vec<Box<Actor>>>> {
    let mut contents = Vec<Vec<Vec<Box<Actor>>>> = vec![];
    for _ in range(0, size.max.x) {
      let mut x_vec: Vec<Vec<Box<Actor>>> = vec![];
      for _ in range(0, size.max.y) {
        let y_vec: Vec<Box<Actor>> = vec![];
        x_vec.push(y_vec);
      }
      contents.push(x_vec);
    }
    return contents;
  }

  pub fn push_actor(&mut self, point: Point, actor: Box<Actor>) {
    self.contents.get_mut(point.x).get_mut(point.y).push(actor);
  }

  pub fn update(&mut self, windows: &mut Windows) {
    let mut new_content = Map::init_contents(self.size);
    for x_iter in self.contents.iter_mut() {
      for y_iter in x_iter.iter_mut() {
        for actor in y_iter.iter_mut() {
          actor.update(windows);
          if actor.is_pc {
            Game::set_character_point(actor.position);
          }
          let point = actor.position;
          let new_actor = actor.clone();
          new_content.get_mut(point.x).get_mut(point.y).push(new_actor);
        }
      }
    }
    self.content = new_content;
  }

  pub fn render(&mut self, renderer: &mut Box<RenderingComponent>) {
    for (x, x_iter) in self.content.iter_mut().enumerate() {
      for (y, y_iter) in x_iter.iter_mut().enumerate() {
        for actor in y_iter.iter_mut() {
          let point = Point::new(x, y);
          renderer.render_object(point, actor.display_char);
        }
      }
    }
  }
}






