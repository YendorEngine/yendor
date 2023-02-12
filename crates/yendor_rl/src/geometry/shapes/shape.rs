use crate::prelude::*;

/// Boxed shape
pub type BoxedShape = Box<dyn Shape>;

/// Boxed shape iterator
pub type BoxedShapeIter = Box<dyn Iterator<Item = IVec2>>;

/// A trait for iterating over the points in a shape
pub trait ShapeIter {
    /// The type of the iterator
    type Iterator: Iterator<Item = IVec2>;

    /// returns an iterator over all points in the shape, inclusively
    fn iter(&self) -> Self::Iterator;
}

/// A trait for dealing with 2D shapes
pub trait Shape {
    /// returns the number of points in the shape
    fn get_count(&self) -> u32;

    /// returns `true` if the point is inside the shape
    fn contains(&self, point: IVec2) -> bool;

    /// returns an iterator over all of the points
    fn get_positions(&self) -> HashSet<IVec2>;

    /// returns a boxed iterator over all of the points
    fn boxed_iter(&self) -> BoxedShapeIter;
}

/// A trait for dealing with 2D shapes with a border
pub trait ShapeWithBorder: Shape {
    /// returns the number of points on the border
    fn get_border_count(&self) -> usize;

    /// returns `true` if the point is inside the shape
    fn border_contains(&self, point: IVec2) -> bool;

    /// returns an iterator over all of the points
    fn get_border_positions(&self) -> HashSet<IVec2>;
}
