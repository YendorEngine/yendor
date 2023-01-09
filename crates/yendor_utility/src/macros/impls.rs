//######################
// Impl Macros
//######################
#[macro_export]
macro_rules! impl_as {
    ($trait:ty, $fn:ident, $return:ty, $cast:ty,$type:ty) => {
        impl $trait for $type {
            fn $fn(&self) -> $return { <$return>::new(self.x as $cast, self.y as $cast) }
        }
    };
}

#[macro_export]
macro_rules! impl_as_array {
    ($trait:ty, $fn:ident, $return:ty, $cast:ty,$type:ty) => {
        impl $trait for $type {
            fn $fn(&self) -> $return { <$return>::new(self[0] as $cast, self[1] as $cast) }
        }
    };
}

#[macro_export]
macro_rules! impl_as_tuple {
    ($trait:ty, $fn:ident, $return:ty, $cast:ty,$type:ty) => {
        impl $trait for $type {
            fn $fn(&self) -> $return { <$return>::new(self.0 as $cast, self.1 as $cast) }
        }
    };
}
