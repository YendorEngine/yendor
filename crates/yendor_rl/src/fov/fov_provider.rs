use crate::prelude::*;

/// A trait for providing information about the field of view.
pub trait FovProvider<T> {
    /// Returns true if the specified position is opaque.
    fn is_opaque(&mut self, position: IVec2, pass_through_data: &mut T) -> bool;
}
