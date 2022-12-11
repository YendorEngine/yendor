/// prelude
pub mod prelude {
    pub use yendor_distance::prelude::*;
    #[cfg(feature = "fov")]
    pub use yendor_fov::prelude::*;
    pub use yendor_geometry::prelude::*;
    pub use yendor_grid::prelude::*;
    #[cfg(feature = "pathfinding")]
    pub use yendor_pathfinding::prelude::*;
    #[cfg(feature = "random")]
    pub use yendor_random::prelude::*;
    pub use yendor_utils::prelude::*;
}

pub mod distance {
    pub use yendor_distance::prelude::*;
}

#[cfg(feature = "fov")]
pub mod fov {
    pub use yendor_fov::prelude::*;
}

pub mod geometry {
    pub use yendor_geometry::prelude::*;
}

pub mod grid {
    pub use yendor_grid::prelude::*;
}

#[cfg(feature = "pathfinding")]
pub mod pathfinding {
    pub use yendor_pathfinding::prelude::*;
}

#[cfg(feature = "random")]
pub mod random {
    pub use yendor_random::prelude::*;
}

pub mod utils {
    pub use yendor_utils::prelude::*;
}
