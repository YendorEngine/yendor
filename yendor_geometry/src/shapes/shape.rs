use crate::prelude::*;

pub type BoxedShape<const DIM: UVec2> = Box<dyn Shape<DIM>>;
pub type BoxedShapeIter<const DIM: UVec2> = Box<dyn Iterator<Item = Position<DIM>>>;

pub trait ShapeIter<const DIM: UVec2> {
    type Iterator: Iterator<Item = Position<DIM>>;

    /// returns an iterator over all points in the shape, inclusively
    fn iter(&self) -> Self::Iterator;
}

pub trait Shape<const DIM: UVec2> {
    /// returns the number of points in the shape
    fn get_count(&self) -> u32;

    /// returns `true` if the point is inside the shape
    fn contains(&self, point: Position<DIM>) -> bool;

    /// returns an iterator over all of the points
    fn get_positions(&self) -> HashSet<Position<DIM>>;

    fn boxed_iter(&self) -> BoxedShapeIter<DIM>;
}

pub trait ShapeWithBorder<const DIM: UVec2>: Shape<DIM> {
    /// returns the number of points on the border
    fn get_border_count(&self) -> usize;

    /// returns `true` if the point is inside the shape
    fn border_contains(&self, point: Position<DIM>) -> bool;

    /// returns an iterator over all of the points
    fn get_border_positions(&self) -> HashSet<Position<DIM>>;
}
