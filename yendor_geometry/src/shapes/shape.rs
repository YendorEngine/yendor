use crate::prelude::*;

pub type BoxedShape<const DIMENSIONS: UVec2> = Box<dyn Shape<DIMENSIONS>>;
pub type BoxedShapeIter<const DIMENSIONS: UVec2> = Box<dyn Iterator<Item = Position<DIMENSIONS>>>;

pub trait ShapeIter<const DIMENSIONS: UVec2> {
    type Iterator: Iterator<Item = Position<DIMENSIONS>>;

    /// returns an iterator over all points in the shape, inclusively
    fn iter(&self) -> Self::Iterator;
}

pub trait Shape<const DIMENSIONS: UVec2> {
    /// returns the number of points in the shape
    fn get_count(&self) -> u32;

    /// returns `true` if the point is inside the shape
    fn contains(&self, point: Position<DIMENSIONS>) -> bool;

    /// returns an iterator over all of the points
    fn get_positions(&self) -> HashSet<Position<DIMENSIONS>>;

    fn boxed_iter(&self) -> BoxedShapeIter<DIMENSIONS>;
}

pub trait ShapeWithBorder<const DIMENSIONS: UVec2>: Shape<DIMENSIONS> {
    /// returns the number of points on the border
    fn get_border_count(&self) -> usize;

    /// returns `true` if the point is inside the shape
    fn border_contains(&self, point: Position<DIMENSIONS>) -> bool;

    /// returns an iterator over all of the points
    fn get_border_positions(&self) -> HashSet<Position<DIMENSIONS>>;
}
