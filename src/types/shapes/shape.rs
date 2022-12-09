use crate::prelude::*;

pub type BoxedShape = Box<dyn Shape>;
pub type BoxedShapeIter = Box<dyn Iterator<Item = Position>>;

pub trait ShapeIter {
    type Iterator: Iterator<Item = Position>;

    /// returns an iterator over all points in the shape, inclusively
    fn iter(&self) -> Self::Iterator;
}

pub trait Shape {
    /// returns the number of points in the shape
    fn get_count(&self) -> u32;

    /// returns `true` if the point is inside the shape
    fn contains(&self, point: Position) -> bool;

    /// returns an iterator over all of the points
    fn get_positions(&self) -> HashSet<Position>;

    fn boxed_iter(&self) -> BoxedShapeIter;
}

pub trait ShapeWithBorder: Shape {
    /// returns the number of points on the border
    fn get_border_count(&self) -> usize;

    /// returns `true` if the point is inside the shape
    fn border_contains(&self, point: Position) -> bool;

    /// returns an iterator over all of the points
    fn get_border_positions(&self) -> HashSet<Position>;
}
