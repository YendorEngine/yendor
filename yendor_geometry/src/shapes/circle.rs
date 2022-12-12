use super::*;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Circle<const DIM: UVec2> {
    center: Position<DIM>,
    radius: u32,
}

impl<const DIM: UVec2> Circle<DIM> {
    pub fn new<R: Into<u32>>(center: Position<DIM>, radius: R) -> Self {
        Self {
            center,
            radius: radius.into(),
        }
    }
}

impl<const DIM: UVec2> Circle<DIM> {
    #[inline]
    pub const fn center(&self) -> Position<DIM> { self.center }

    #[inline]
    pub fn left(&self) -> Position<DIM> { self.center - IVec2::new(self.radius as i32, 0) }

    #[inline]
    pub fn right(&self) -> Position<DIM> { self.center + IVec2::new(self.radius as i32, 0) }

    #[inline]
    pub fn top(&self) -> Position<DIM> { self.center + IVec2::new(0, self.radius as i32) }

    #[inline]
    pub fn bottom(&self) -> Position<DIM> { self.center + IVec2::new(0, self.radius as i32) }

    #[inline]
    pub fn as_horizontal_line(&self) -> Line<DIM> { Line::new(self.left(), self.right()) }

    #[inline]
    pub fn as_vertical_line(&self) -> Line<DIM> { Line::new(self.bottom(), self.top()) }
}

impl<const DIM: UVec2> Shape<DIM> for Circle<DIM> {
    #[inline]
    // FIX: PERF??
    fn get_count(&self) -> u32 { self.get_positions().len() as u32 }

    #[inline]
    fn contains(&self, position: Position<DIM>) -> bool { self.get_positions().contains(&position) }

    // TODO: Move to iter()
    #[inline]
    fn get_positions(&self) -> HashSet<Position<DIM>> {
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
    fn boxed_iter(&self) -> BoxedShapeIter<DIM> { Box::new(self.into_iter()) }
}

impl<const DIM: UVec2> ShapeIter<DIM> for Circle<DIM> {
    type Iterator = bevy::utils::hashbrown::hash_set::IntoIter<Position<DIM>>;

    #[inline]
    fn iter(&self) -> Self::Iterator { self.get_positions().into_iter() }
}

impl<const DIM: UVec2> IntoIterator for Circle<DIM> {
    type IntoIter = bevy::utils::hashbrown::hash_set::IntoIter<Position<DIM>>;
    type Item = Position<DIM>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.get_positions().into_iter() }
}

impl<const DIM: UVec2> From<Circle<DIM>> for BoxedShape<DIM> {
    fn from(value: Circle<DIM>) -> Self { Box::new(value) }
}
