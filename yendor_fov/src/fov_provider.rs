use crate::prelude::*;

pub trait FovProvider<T, const DIM: UVec2> {
    fn is_opaque(&mut self, position: Position<DIM>, pass_through_data: &mut T) -> bool;
}
