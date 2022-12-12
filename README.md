Yendor: a set of tools for roguelike developers using Bevy.

Yendor provides tools for 


Many of the types inside of Yendor use `const generics` in order to keep track of their maximum sizes. As such, we find it cleaner to set this maximum size upon importing Yendor. You may type these for different sizes, but the sizes must all be known at compile time. (eg: small maps will be 128x128 tiles, medium maps will be 256x256 tiles, and large maps will be 512x512 tiles). We like to set our grid grid to the visible size on screen.

For a single grid size, create a const `GRID_SIZE`, of type `UVec2` from the Bevy crate, before importing Yendor into your project.
NOTE: If you keep track of your visible grid size in a constant elsewhere, make sure to add a `use` for that value above importing the Yendor crate. This will replace the value for `GRID_SIZE` in our example.
```
pub const GRID_SIZE: bevy::prelude::UVec2 = bevy::prelude::UVec2::new(GRID_WIDTH, GRID_HEIGHT);
```

Next we will import many of the types from the prelude under a temporary name:
```
pub use yendor::prelude::{
    Grid as YendorGrid,
    Position as YendorPosition,
    Random as YendorRandom,
    Shape as YendorShape,
    Rectangle as YendorRectangle,
    Circle as YendorCircle,
    Line as YendorLine,
};
```

Now we will use the `GRID_SIZE` to retype each of these so that they all talk to each other without having to remind them what size they are throughout the code.
```
pub type Grid<T> = YendorGrid<T, GRID_SIZE>;
pub type Position = YendorPosition<GRID_SIZE>;
pub trait Shape = YendorShape<GRID_SIZE>;
pub type GridRectangle = YendorRectangle<GRID_SIZE>;
pub type Circle = YendorCircle<GRID_SIZE>;
pub type Line = YendorLine<GRID_SIZE>;
```


Last we want to finish importing the rest of the prelude, and altogether looks similar to the following:
```
pub const GRID_SIZE: bevy::prelude::UVec2 = bevy::prelude::UVec2::new(GRID_WIDTH, GRID_HEIGHT);

pub use yendor::prelude::{
    Grid as YendorGrid,
    Position as YendorPosition,
    Random as YendorRandom,
    Shape as YendorShape,
    Rectangle as YendorRectangle,
    Circle as YendorCircle,
    Line as YendorLine,
    *,
};

pub type Grid<T> = YendorGrid<T, GRID_SIZE>;
pub type Position = YendorPosition<GRID_SIZE>;
pub trait Shape = YendorShape<GRID_SIZE>;
pub type GridRectangle = YendorRectangle<GRID_SIZE>;
pub type Circle = YendorCircle<GRID_SIZE>;
pub type Line = YendorLine<GRID_SIZE>;
```
