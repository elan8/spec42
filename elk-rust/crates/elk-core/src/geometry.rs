#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    #[must_use]
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    #[must_use]
    pub const fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }

    #[must_use]
    pub fn center(self) -> Point {
        Point::new(
            self.origin.x + self.size.width / 2.0,
            self.origin.y + self.size.height / 2.0,
        )
    }

    #[must_use]
    pub fn max_x(self) -> f32 {
        self.origin.x + self.size.width
    }

    #[must_use]
    pub fn max_y(self) -> f32 {
        self.origin.y + self.size.height
    }

    #[must_use]
    pub fn intersects(self, other: Self) -> bool {
        self.origin.x < other.max_x()
            && self.max_x() > other.origin.x
            && self.origin.y < other.max_y()
            && self.max_y() > other.origin.y
    }
}
