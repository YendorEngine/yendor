use crate::prelude::*;

pub trait FovProvider<T> {
    fn is_opaque(&mut self, position: IVec2, pass_through_data: &mut T) -> bool;
}
