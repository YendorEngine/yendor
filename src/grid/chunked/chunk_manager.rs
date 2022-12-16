use crate::prelude::*;

pub type ChunkWorldPosition = IVec3;
pub type ChunkDimensions = UVec2;
pub type ChunkLocalPosition = UVec2;

pub struct ChunkManager<T> {
    dimensions: ChunkDimensions,
    chunk_provider: Box<dyn ChunkProvider<T>>,
    loaded_chunks: HashMap<ChunkWorldPosition, Chunk<T>>,
}

unsafe impl<T> Send for ChunkManager<T> {}
unsafe impl<T> Sync for ChunkManager<T> {}

// Constructor
//##################
impl<T> ChunkManager<T> {
    pub fn new<D: Into<ChunkDimensions>>(
        dimensions: D,
        chunk_provider: impl ChunkProvider<T> + 'static,
    ) -> Self {
        Self {
            dimensions: dimensions.into(),
            chunk_provider: Box::new(chunk_provider),

            loaded_chunks: HashMap::new(),
        }
    }
}

// Position Constructor
//##################
impl<T> ChunkManager<T> {
    pub fn new_position(
        &self,
        world_position: ChunkWorldPosition,
        local_position: ChunkLocalPosition,
    ) -> ChunkPosition {
        ChunkPosition::new(
            world_position.x as i64 * self.dimensions.x as i64 + local_position.x as i64,
            world_position.y as i64 * self.dimensions.y as i64 + local_position.y as i64,
            world_position.z,
        )
    }
}

// World/Local Extraction
//##################
impl<T> ChunkManager<T> {
    pub fn position(&self, position: ChunkPosition) -> (ChunkWorldPosition, ChunkLocalPosition) {
        let position_x = position.abs_x();
        let position_y = position.abs_y();

        let (world_x, local_x) = if position_x < 0 {
            let abs_x = position_x.abs();
            let mut world = abs_x / self.dimensions.x as i64;
            let mut local = self.dimensions.x as i64 - (abs_x - (world * self.dimensions.x as i64));
            if local == self.dimensions.x as i64 {
                world -= 1;
                local = 0;
            }
            (-(world as i32) - 1, local as u32)
        } else {
            (
                (position_x / self.dimensions.x as i64) as i32,
                (position_x % self.dimensions.x as i64) as u32,
            )
        };

        let (world_y, local_y) = if position_y < 0 {
            let abs_y = position_y.abs();
            let mut world = abs_y / self.dimensions.y as i64;
            let mut local = self.dimensions.y as i64 - (abs_y - (world * self.dimensions.y as i64));
            if local == self.dimensions.y as i64 {
                world -= 1;
                local = 0;
            }
            (-(world as i32) - 1, local as u32)
        } else {
            (
                (position_y / self.dimensions.y as i64) as i32,
                (position_y % self.dimensions.y as i64) as u32,
            )
        };

        (
            IVec3::new(world_x, world_y, position.abs_z()),
            UVec2::new(local_x, local_y),
        )
    }

    pub fn world_position(&self, position: ChunkPosition) -> ChunkWorldPosition { self.position(position).0 }

    pub fn local_position(&self, position: ChunkPosition) -> ChunkLocalPosition { self.position(position).1 }
}

// Getters
//##################
impl<T> ChunkManager<T> {
    pub fn dimensions(&self) -> ChunkDimensions { self.dimensions }

    pub fn width(&self) -> u32 { self.dimensions.x }

    pub fn height(&self) -> u32 { self.dimensions.y }
}

// Load/Store Functions
//##################
impl<T> ChunkManager<T> {
    pub fn is_loaded_at(&self, position: ChunkPosition) -> bool {
        self.is_loaded(self.world_position(position))
    }

    pub fn is_loaded(&self, world_position: ChunkWorldPosition) -> bool {
        self.loaded_chunks.contains_key(&world_position)
    }

    pub fn load_at(&mut self, position: ChunkPosition) { self.load(self.world_position(position)) }

    pub fn load(&mut self, world_position: ChunkWorldPosition) {
        if !self.is_loaded(world_position) {
            let cells = self.chunk_provider.load(world_position, self.dimensions);
            self.loaded_chunks.insert(world_position, Chunk::new(self.dimensions, cells));
        }
    }

    pub fn store_at(&mut self, position: ChunkPosition) { self.store(self.world_position(position)) }

    pub fn store(&mut self, world_position: ChunkWorldPosition) {
        if let Some(chunk) = self.loaded_chunks.remove(&world_position) {
            self.chunk_provider.store(world_position, chunk.remove_cells());
        }
    }
}

// get<T> Functions
//##################
impl<T> ChunkManager<T> {
    pub fn get(&mut self, position: ChunkPosition) -> &T {
        self.load_at(position);
        self.get_no_load(position).expect("Chunk failed to load.")
    }

    pub fn get_no_load(&self, position: ChunkPosition) -> Option<&T> {
        let (world_position, local_position) = self.position(position);
        if let Some(chunk) = self.get_chunk(world_position) {
            Some(chunk.get(local_position))
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, position: ChunkPosition) -> &mut T {
        let (world_position, local_position) = self.position(position);
        self.load(world_position);
        let chunk = self.get_chunk_mut(world_position).expect("Chunk failed to load.");
        chunk.get_mut(local_position)
    }

    pub fn set(&mut self, position: ChunkPosition, value: T) -> T {
        let (world_position, local_position) = self.position(position);
        self.load(world_position);
        let chunk = self.get_chunk_mut(world_position).expect("Chunk failed to load.");
        chunk.set(local_position, value)
    }

    // pub fn iter(&self, shape: Box<dyn Shape>) -> ChunkIter<T>;
}

// Private
//##################
impl<T> ChunkManager<T> {
    fn get_chunk(&self, world_position: ChunkWorldPosition) -> Option<&Chunk<T>> {
        self.loaded_chunks.get(&world_position)
    }

    fn get_chunk_mut(&mut self, world_position: ChunkWorldPosition) -> Option<&mut Chunk<T>> {
        self.loaded_chunks.get_mut(&world_position)
    }
}

// pub struct ChunkIter<T> {
//}
// impl<T> Iterator for ChunkIter<T> {
//    type Item = T;
//    fn next(&mut self) -> Option<Self::Item> {
//
//    }
//}
