use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Circle {
    center: Position,
    radius: u32,
}

impl Circle {
    pub fn new<R: Into<u32>>(center: Position, radius: R) -> Self {
        Self {
            center,
            radius: radius.into(),
        }
    }
}

impl Circle {
    #[inline]
    pub const fn center(&self) -> Position { self.center }

    #[inline]
    pub fn left(&self) -> Position { self.center - IVec2::new(self.radius as i32, 0) }

    #[inline]
    pub fn right(&self) -> Position { self.center + IVec2::new(self.radius as i32, 0) }

    #[inline]
    pub fn top(&self) -> Position { self.center + IVec2::new(0, self.radius as i32) }

    #[inline]
    pub fn bottom(&self) -> Position { self.center + IVec2::new(0, self.radius as i32) }

    #[inline]
    pub fn as_horizontal_line(&self) -> Line { Line::new(self.left(), self.right()) }

    #[inline]
    pub fn as_vertical_line(&self) -> Line { Line::new(self.bottom(), self.top()) }
}

impl Shape for Circle {
    #[inline]
    // FIX: PERF??
    fn get_count(&self) -> u32 { self.get_positions().len() as u32 }

    #[inline]
    fn contains(&self, position: Position) -> bool { self.get_positions().contains(&position) }

    // TODO: Move to iter()
    #[inline]
    fn get_positions(&self) -> HashSet<Position> {
        let mut discovered = HashSet::new();
        let mut d = (5 - (self.radius as i32 * 4)) / 4;
        let mut x = 0;
        let mut y = self.radius as i32;

        let mut start;
        let mut end;
        let mut line;
        loop {
            start = self.center + IVec2::new(x, y);
            end = self.center + IVec2::new(x, -y);
            line = Line::new(start, end);
            for point in line.get_positions() {
                discovered.insert(point);
            }

            start = self.center + IVec2::new(-x, y);
            end = self.center + IVec2::new(-x, -y);
            line = Line::new(start, end);
            for point in line.get_positions() {
                discovered.insert(point);
            }

            start = self.center + IVec2::new(y, x);
            end = self.center + IVec2::new(y, -x);
            line = Line::new(start, end);
            for point in line.get_positions() {
                discovered.insert(point);
            }

            start = self.center + IVec2::new(-y, x);
            end = self.center + IVec2::new(-y, -x);
            line = Line::new(start, end);
            for point in line.get_positions() {
                discovered.insert(point);
            }

            //// Circumfrence
            // discovered.insert(IVec2::new(self.center.x + x, self.center.y + y));
            // discovered.insert(IVec2::new(self.center.x + x, self.center.y - y));
            // discovered.insert(IVec2::new(self.center.x - x, self.center.y + y));
            // discovered.insert(IVec2::new(self.center.x - x, self.center.y - y));
            // discovered.insert(IVec2::new(self.center.x + y, self.center.y + x));
            // discovered.insert(IVec2::new(self.center.x + y, self.center.y - x));
            // discovered.insert(IVec2::new(self.center.x - y, self.center.y + x));
            // discovered.insert(IVec2::new(self.center.x - y, self.center.y - x));

            if d < 0 {
                d += (2 * x) + 1;
            } else {
                d += (2 * (x - y)) + 1;
                y -= 1;
            }
            x += 1;

            if x > y {
                break;
            }
        }
        discovered
    }

    #[inline]
    fn boxed_iter(&self) -> BoxedShapeIter { Box::new(self.into_iter()) }
}

impl ShapeIter for Circle {
    type Iterator = bevy::utils::hashbrown::hash_set::IntoIter<Position>;

    #[inline]
    fn iter(&self) -> Self::Iterator { self.get_positions().into_iter() }
}

impl IntoIterator for Circle {
    type IntoIter = bevy::utils::hashbrown::hash_set::IntoIter<Position>;
    type Item = Position;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.get_positions().into_iter() }
}

impl From<Circle> for BoxedShape {
    fn from(value: Circle) -> Self { Box::new(value) }
}
