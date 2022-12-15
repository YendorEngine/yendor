use crate::prelude::*;

pub trait GridLayer<T> {
    type MutableReturn<'a>
    where
        T: 'a,
        Self: 'a;

    /// Create a new grid layer with the given size (clone the given value).
    fn new_clone(size: impl Dimensions, value: T) -> Self
    where T: Clone;

    /// Clone a region of the grid onto a new grid.
    fn blit_clone(&mut self, to: impl Point, source: &Self, from: impl Point, size: impl Dimensions)
    where T: Clone;

    /// Create a new grid with the given size (copy the given value).
    fn new_copy(size: impl Dimensions, value: T) -> Self
    where T: Copy;

    /// Copy a region from another grid into this grid.
    fn blit_copy(&mut self, to: impl Point, source: &Self, from: impl Point, size: impl Dimensions)
    where T: Copy;

    /// Create a new grid with the given size and default values.
    fn new_default(size: impl Dimensions) -> Self
    where T: Default;

    fn new_fn(size: impl Dimensions, f: impl Fn(IVec2) -> T) -> Self;

    ///////////////////////////////////////////////////////////////////////////
    // Utility Functionality
    ///////////////////////////////////////////////////////////////////////////
    /// Returns the size of the grid.
    fn width(&self) -> u32;

    /// Returns the height of the grid.
    fn height(&self) -> u32;

    /// Returns the size of the grid as a 2D vector.
    fn dimensions(&self) -> UVec2;

    /// Returns the number of elements in the vector, also referred to as its 'length'.
    fn len(&self) -> usize;

    /// Returns true if the vector contains no elements
    fn is_empty(&self) -> bool;

    /// Tests whether a point is in bounds.
    fn in_bounds(&self, pos: impl Point) -> bool;

    ///////////////////////////////////////////////////////////////////////////
    // Getter/Setter Functionality
    ///////////////////////////////////////////////////////////////////////////
    /// Try Gets the `Point` corresponding to an index
    ///
    /// Returns `None` if the index is out of bounds.
    fn get_idx(&self, pos: impl Point) -> Option<usize>;

    /// Gets the index corresponding to a coordinate, which is row-wise.
    ///
    /// # Panic
    ///
    /// Panics if the coordinate is out of bounds.
    fn get_idx_unchecked(&self, point: impl Point) -> usize;

    /// Try Gets the `Point` corresponding to an index
    ///
    /// Returns `None` if the index is out of bounds.
    fn index_to_pt(&self, idx: usize) -> Option<IVec2>;

    /// Gets the `Point` corresponding to an index
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn index_to_pt_unchecked(&self, idx: usize) -> IVec2;

    /// Gets a reference to the element at the given index.
    fn get(&self, pos: impl Point) -> Option<&T>;

    /// Gets a mutable reference to the element at the given index.
    fn get_mut(&mut self, pos: impl Point) -> Option<Self::MutableReturn<'_>>;

    /// Gets a reference to the element at the given index (unchecked).
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn get_unchecked(&self, pos: impl Point) -> &T;

    /// Gets a mutable reference to the element at the given index (unchecked).
    fn get_mut_unchecked(&mut self, pos: impl Point) -> Self::MutableReturn<'_>;

    /// Sets the value of the element at the given index.
    fn set(&mut self, pos: impl Point, value: T) -> Option<T>;

    /// Sets the value of the element at the given index (unchecked).
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn set_unchecked(&mut self, pos: impl Point, value: T) -> T;
}
