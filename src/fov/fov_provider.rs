use crate::prelude::*;

pub trait FovProvider<T> {
    fn is_opaque(&mut self, position: ChunkPosition, pass_through_data: &mut T) -> bool;
}
