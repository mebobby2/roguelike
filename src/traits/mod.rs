use rendering::RenderingComponent;

pub trait Updates {
    fn update(&mut self);
    fn render(&self, &mut Box<RenderingComponent>);
}