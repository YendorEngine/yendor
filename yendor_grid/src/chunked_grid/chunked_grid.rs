use crate::prelude::*;

pub trait LoadChunk<T> = Fn(IVec3) -> Grid<T>;
pub trait StoreChunk<T> = Fn(IVec3, Grid<T>);

pub struct ChunkedGrid<'f, T> {
    dimensions: UVec2,
    load_chunk: Box<&'f dyn LoadChunk<T>>,
    store_chunk: Box<&'f dyn StoreChunk<T>>,

    //max_loaded_grids: u32,
    loaded_positions: VecDeque<IVec3>,
    loaded_chunks: HashMap<IVec3, Grid<T>>
}

// Constructor
//######################
impl<'f, T> ChunkedGrid<'f, T> {
    pub fn new<D: Into<UVec2>>(chunk_dimensions: D, load_chunk: &'f dyn LoadChunk<T>, store_chunk: &'f dyn StoreChunk<T>) -> Self {
        Self {
            dimensions: chunk_dimensions.into(),
            load_chunk: Box::new(load_chunk),
            store_chunk: Box::new(store_chunk),

            //max_loaded_grids: 0,
            loaded_positions: VecDeque::new(),
            loaded_chunks: HashMap::new(),
        }
    }
}

// Position
//######################
impl<'f, T> ChunkedGrid<'f, T> {
    pub fn position(&self, chunk_position: IVec3, local_position: UVec2) -> ChunkPosition {
        ChunkPosition::new_dimensions(chunk_position, local_position, self.dimensions)
    }
}

// Public
//######################
impl<'f, T> ChunkedGrid<'f, T> {
    pub fn chunk_width(&self) -> u32 {
        self.dimensions.x
    }

    pub fn chunk_height(&self) -> u32 {
        self.dimensions.y
    }

    pub fn chunk_dimensions(&self) -> UVec2 {
        self.dimensions
    }

    pub fn is_loaded(&self, point: ChunkPosition) -> bool {
        let chunk_position = point.chunk_position(self.dimensions);
        self.is_loaded_chunk_position(chunk_position)
    }

    pub fn load(&mut self, point: ChunkPosition) {
        let chunk_position = point.chunk_position(self.dimensions);
        self.verify_loaded(chunk_position)
    }

    pub fn store(&mut self, point: ChunkPosition) {
        let chunk_position = point.chunk_position(self.dimensions);
        self.store_chunk(chunk_position);
    }

    pub fn store_all(&mut self) {
        loop {
            if let Some(chunk_position) = self.loaded_positions.front() {
                self.store_chunk(*chunk_position);
            } else {
                break;
            }
        }
    }

    pub fn get(&mut self, point: ChunkPosition) -> &T {
        self.load(point);
        self.get_no_load(point).unwrap()
    }

    pub fn get_no_load(&self, point: ChunkPosition) -> Option<&T> {
        let chunk_position = point.position(self.dimensions);
        if let Some(chunk) = self.get_chunk(chunk_position.0) {
            Some(chunk.get_unchecked(chunk_position.1))
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, point: ChunkPosition) -> &mut T {
        let chunk_position = point.position(self.dimensions);
        self.verify_loaded(chunk_position.0);
        let chunk = self.get_chunk_mut(chunk_position.0).unwrap();
        chunk.get_mut_unchecked(chunk_position.1)
    }

    pub fn set(&mut self, point: ChunkPosition, value: T) -> T {
        let chunk_position = point.position(self.dimensions);
        self.verify_loaded(chunk_position.0);
        let chunk = self.get_chunk_mut(chunk_position.0).unwrap();
        chunk.set_unchecked(chunk_position.1, value)
    }

    //pub fn iter(&mut self, shape: impl Shape)
    //pub fn iter_mut(&mut self, shape: impl Shape)
    //pub fn iter_set<F: Fn(IVec3, IVec2, Option<T>) -> T>(&mut self, shape: impl Shape, set_function: F)
}

// Private
//######################
impl<'f, T> ChunkedGrid<'f, T> {
    fn is_loaded_chunk_position(&self, chunk_position: IVec3) -> bool {
        self.loaded_positions.contains(&chunk_position)
    }

    fn verify_loaded(&mut self, chunk_position: IVec3) {
        if !self.is_loaded_chunk_position(chunk_position) {
            let chunk:Grid<T> = (self.load_chunk)(chunk_position);
            self.loaded_positions.push_back(chunk_position);
            self.loaded_chunks.insert(chunk_position, chunk);
        }
    }

    fn get_chunk(&self, chunk_position: IVec3) -> Option<&Grid<T>> {
        self.loaded_chunks.get(&chunk_position)
    }

    fn get_chunk_mut(&mut self, chunk_position: IVec3) -> Option<&mut Grid<T>> {
        self.loaded_chunks.get_mut(&chunk_position)
    }

    fn store_chunk(&mut self, chunk_position: IVec3) {
        if let Some(chunk) = self.loaded_chunks.remove(&chunk_position) {
            self.loaded_positions.retain(|vec| *vec != chunk_position);
            (self.store_chunk)(chunk_position, chunk);
        }
    }
}