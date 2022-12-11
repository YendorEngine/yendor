use crate::prelude::*;

pub trait FovProvider<T, const DIMENSIONS: UVec2> {
    fn is_opaque(&mut self, position: Position<DIMENSIONS>, pass_through_data: &mut T) -> bool;
}
