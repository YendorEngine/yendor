use crate::prelude::*;

//######################
// Impl Macros
//######################
macro_rules! impl_as {
    ($trait:ty, $fn:ident, $return:ty, $cast:ty,$type:ty) => {
        impl $trait for $type {
            fn $fn(&self) -> $return { <$return>::new(self.x as $cast, self.y as $cast) }
        }
    };
}

macro_rules! impl_as_array {
    ($trait:ty, $fn:ident, $return:ty, $cast:ty,$type:ty) => {
        impl $trait for $type {
            fn $fn(&self) -> $return { <$return>::new(self[0] as $cast, self[1] as $cast) }
        }
    };
}

macro_rules! impl_as_tuple {
    ($trait:ty, $fn:ident, $return:ty, $cast:ty,$type:ty) => {
        impl $trait for $type {
            fn $fn(&self) -> $return { <$return>::new(self.0 as $cast, self.1 as $cast) }
        }
    };
}

//######################
// UVec2
//######################
pub trait ToUVec2 {
    fn as_uvec2(&self) -> UVec2;
}

// Bevy Math Structs
impl_as!(ToUVec2, as_uvec2, UVec2, u32, Vec2);
impl_as!(ToUVec2, as_uvec2, UVec2, u32, IVec2);
impl_as_array!(ToUVec2, as_uvec2, UVec2, u32, [i32; 2]);
impl_as_tuple!(ToUVec2, as_uvec2, UVec2, u32, (i32, i32));
impl_as_array!(ToUVec2, as_uvec2, UVec2, u32, [u32; 2]);
impl_as_tuple!(ToUVec2, as_uvec2, UVec2, u32, (u32, u32));
impl_as_array!(ToUVec2, as_uvec2, UVec2, u32, [usize; 2]);
impl_as_tuple!(ToUVec2, as_uvec2, UVec2, u32, (usize, usize));

//######################
// IVec2
//######################
pub trait ToIVec2 {
    fn as_ivec2(&self) -> IVec2;
}

// Bevy Math Structs
impl_as!(ToIVec2, as_ivec2, IVec2, i32, Vec2);
impl_as!(ToIVec2, as_ivec2, IVec2, i32, UVec2);
impl_as_array!(ToIVec2, as_ivec2, IVec2, i32, [i32; 2]);
impl_as_tuple!(ToIVec2, as_ivec2, IVec2, i32, (i32, i32));
impl_as_array!(ToIVec2, as_ivec2, IVec2, i32, [u32; 2]);
impl_as_tuple!(ToIVec2, as_ivec2, IVec2, i32, (u32, u32));
impl_as_array!(ToIVec2, as_ivec2, IVec2, i32, [usize; 2]);
impl_as_tuple!(ToIVec2, as_ivec2, IVec2, i32, (usize, usize));

//######################
// Vec2
//######################

pub trait ToVec2 {
    fn as_vec2(&self) -> Vec2;
}

// Bevy Math Structs
impl_as!(ToVec2, as_vec2, Vec2, f32, IVec2);
impl_as!(ToVec2, as_vec2, Vec2, f32, UVec2);
impl_as_array!(ToVec2, as_vec2, Vec2, f32, [i32; 2]);
impl_as_tuple!(ToVec2, as_vec2, Vec2, f32, (i32, i32));
impl_as_array!(ToVec2, as_vec2, Vec2, f32, [u32; 2]);
impl_as_tuple!(ToVec2, as_vec2, Vec2, f32, (u32, u32));
impl_as_array!(ToVec2, as_vec2, Vec2, f32, [usize; 2]);
impl_as_tuple!(ToVec2, as_vec2, Vec2, f32, (usize, usize));
