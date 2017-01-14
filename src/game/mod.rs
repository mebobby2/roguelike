use util::Bound;

#[derive(Copy, Clone)]
pub struct Game {
    pub exit: bool,
    pub window_bounds: Bound
}
