use crate::prelude::*;

pub type BoxedShape<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> = Box<dyn Shape<GRID_WIDTH, GRID_HEIGHT>>;
pub type BoxedShapeIter<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> =
    Box<dyn Iterator<Item = Position<GRID_WIDTH, GRID_HEIGHT>>>;

pub trait ShapeIter<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> {
    type Iterator: Iterator<Item = Position<GRID_WIDTH, GRID_HEIGHT>>;

    /// returns an iterator over all points in the shape, inclusively
    fn iter(&self) -> Self::Iterator;
}

pub trait Shape<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> {
    /// returns the number of points in the shape
    fn get_count(&self) -> u32;

    /// returns `true` if the point is inside the shape
    fn contains(&self, point: Position<GRID_WIDTH, GRID_HEIGHT>) -> bool;

    /// returns an iterator over all of the points
    fn get_positions(&self) -> HashSet<Position<GRID_WIDTH, GRID_HEIGHT>>;

    fn boxed_iter(&self) -> BoxedShapeIter<GRID_WIDTH, GRID_HEIGHT>;
}

pub trait ShapeWithBorder<const GRID_WIDTH: u32, const GRID_HEIGHT: u32>:
    Shape<GRID_WIDTH, GRID_HEIGHT>
{
    /// returns the number of points on the border
    fn get_border_count(&self) -> usize;

    /// returns `true` if the point is inside the shape
    fn border_contains(&self, point: Position<GRID_WIDTH, GRID_HEIGHT>) -> bool;

    /// returns an iterator over all of the points
    fn get_border_positions(&self) -> HashSet<Position<GRID_WIDTH, GRID_HEIGHT>>;
}
