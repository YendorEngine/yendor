use crate::prelude::*;
#[macro_export]
macro_rules! impl_grid_point_array {
    ($type:ty) => {
        impl Point for $type {
            fn x_int32(&self) -> i32 {
                self.x as i32
            }

            fn y_int32(&self) -> i32 {
                self.y as i32
            }

            fn x_uint32(&self) -> u32 {
                self.x as u32
            }

            fn y_uint32(&self) -> u32 {
                self.y as u32
            }

            fn x_float32(&self) -> f32 {
                self.x as f32
            }

            fn y_float32(&self) -> f32 {
                self.y as f32
            }
        }
    };
}
#[macro_export]
macro_rules! impl_grid_point_tuple {
    ($type:ty) => {
        impl Point for $type {
            fn x_int32(&self) -> i32 {
                self.0 as i32
            }

            fn y_int32(&self) -> i32 {
                self.1 as i32
            }

            fn x_uint32(&self) -> u32 {
                self.0 as u32
            }

            fn y_uint32(&self) -> u32 {
                self.1 as u32
            }

            fn x_float32(&self) -> f32 {
                self.0 as f32
            }

            fn y_float32(&self) -> f32 {
                self.1 as f32
            }
        }
    };
}
impl_grid_point_array!(IVec2);
impl_grid_point_array!(UVec2);
impl_grid_point_tuple!((u32, u32));
impl_grid_point_tuple!((i32, i32));
impl_grid_point_tuple!((usize, usize));

// Vec2
impl Point for Vec2 {
    fn x_int32(&self) -> i32 {
        self.x.floor() as i32
    }

    fn y_int32(&self) -> i32 {
        self.y.floor() as i32
    }

    fn x_uint32(&self) -> u32 {
        self.x.floor() as u32
    }

    fn y_uint32(&self) -> u32 {
        self.y.floor() as u32
    }

    fn x_float32(&self) -> f32 {
        self.x
    }

    fn y_float32(&self) -> f32 {
        self.y
    }
}

// (f32, f32)
impl Point for (f32, f32) {
    fn x_int32(&self) -> i32 {
        self.0.floor() as i32
    }

    fn y_int32(&self) -> i32 {
        self.1.floor() as i32
    }

    fn x_uint32(&self) -> u32 {
        self.0.floor() as u32
    }

    fn y_uint32(&self) -> u32 {
        self.1.floor() as u32
    }

    fn x_float32(&self) -> f32 {
        self.0
    }

    fn y_float32(&self) -> f32 {
        self.1
    }
}

// TODO: Hide behind feature and uncomment
// TilePos
//impl Point for TilePos {
//    fn x(&self) -> i32 { self.x as i32 }
//
//    fn y(&self) -> i32 { self.y as i32 }
//}
