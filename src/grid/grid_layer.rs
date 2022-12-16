use crate::prelude::*;

pub trait GridLayer<T> {
    type MutableReturn<'a>
    where
        T: 'a,
        Self: 'a;

    /// Create a new grid layer with the given size (clone the given value).
    fn new_clone(size: UVec2, value: T) -> Self
    where T: Clone;

    /// Clone a region of the grid onto a new grid.
    fn blit_clone(&mut self, to: UVec2, source: &Self, from: UVec2, size: UVec2)
    where T: Clone;

    /// Create a new grid with the given size (copy the given value).
    fn new_copy(size: UVec2, value: T) -> Self
    where T: Copy;

    /// Copy a region from another grid into this grid.
    fn blit_copy(&mut self, to: UVec2, source: &Self, from: UVec2, size: UVec2)
    where T: Copy;

    /// Create a new grid with the given size and default values.
    fn new_default(size: UVec2) -> Self
    where T: Default;

    /// Create a new grid with the given size and a function to generate values.
    fn new_fn(size: UVec2, f: impl Fn(UVec2) -> T) -> Self;

    /// Create a new grid with the given size and a vector of values.
    fn from_vec(size: UVec2, cells: Vec<T>) -> Self;

    ///////////////////////////////////////////////////////////////////////////
    // Utility Functionality
    ///////////////////////////////////////////////////////////////////////////
    /// Returns the size of the grid.
    fn width(&self) -> u32;

    /// Returns the height of the grid.
    fn height(&self) -> u32;

    /// Returns the encapsulated cells of the grid
    fn take(&mut self) -> Vec<T>;

    /// Returns the size of the grid as a 2D vector.
    fn dimensions(&self) -> UVec2;

    /// Returns the number of elements in the vector, also referred to as its 'length'.
    fn len(&self) -> usize;

    /// Returns true if the vector contains no elements
    fn is_empty(&self) -> bool;

    /// Tests whether a point is in bounds.
    fn in_bounds(&self, pos: UVec2) -> bool;

    ///////////////////////////////////////////////////////////////////////////
    // Getter/Setter Functionality
    ///////////////////////////////////////////////////////////////////////////
    /// Try Gets the `Point` corresponding to an index
    ///
    /// Returns `None` if the index is out of bounds.
    fn get_idx(&self, pos: UVec2) -> Option<usize>;

    /// Gets the index corresponding to a coordinate, which is row-wise.
    ///
    /// # Panic
    ///
    /// Panics if the coordinate is out of bounds.
    fn get_idx_unchecked(&self, point: UVec2) -> usize;

    /// Try Gets the `Point` corresponding to an index
    ///
    /// Returns `None` if the index is out of bounds.
    fn index_to_pt(&self, idx: usize) -> Option<UVec2>;

    /// Gets the `Point` corresponding to an index
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn index_to_pt_unchecked(&self, idx: usize) -> UVec2;

    /// Gets a reference to the element at the given index.
    fn get(&self, pos: UVec2) -> Option<&T>;

    /// Gets a mutable reference to the element at the given index.
    fn get_mut(&mut self, pos: UVec2) -> Option<Self::MutableReturn<'_>>;

    /// Gets a reference to the element at the given index (unchecked).
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn get_unchecked(&self, pos: UVec2) -> &T;

    /// Gets a mutable reference to the element at the given index (unchecked).
    fn get_mut_unchecked(&mut self, pos: UVec2) -> Self::MutableReturn<'_>;

    /// Sets the value of the element at the given index.
    fn set(&mut self, pos: UVec2, value: T) -> Option<T>;

    /// Sets the value of the element at the given index (unchecked).
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    fn set_unchecked(&mut self, pos: UVec2, value: T) -> T;
}
