#[derive(Copy, Clone)]
pub struct Point {
  pub x: i32,
  pub y: i32
}

impl Point {
    pub fn offset_x(&self, offset: i32) -> Point {
        Point { x: self.x + offset, .. *self }
    }

    pub fn offset_y(&self, offset: i32) -> Point {
        Point { y: self.y + offset, .. *self }
    }

    pub fn offset(&self, offset: Point) -> Point {
        Point { x: self.x + offset.x, y: self.y + offset.y }
    }
}

pub enum Contains {
    DoesContain,
    DoesNotContain
}

#[derive(Copy, Clone)]
pub struct Bound {
    pub min: Point,
    pub max: Point
}

impl Bound {
    pub fn contains(&self, point: Point) -> Contains {
        if
            point.x >= self.min.x &&
            point.x < self.max.x &&
            point.y >= self.min.y &&
            point.y < self.max.y
        {
            Contains::DoesContain
        } else {
            Contains::DoesNotContain
        }
    }
}