#[cfg(feature = "reflect")]
use crate::prelude::*;

#[cfg(not(feature = "reflect"))]
pub trait GridParam: Sync + Send + 'static {}

#[cfg(not(feature = "reflect"))]
impl<T: Sync + Send + 'static> GridParam for T {}

#[cfg(feature = "reflect")]
pub trait GridParam: Sync + Send + 'static + FromReflect {}

#[cfg(feature = "reflect")]
impl<T: FromReflect> GridParam for T {}
