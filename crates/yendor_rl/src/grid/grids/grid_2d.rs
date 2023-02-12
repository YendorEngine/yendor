use crate::prelude::*;

pub type GridIter<'a, T> = slice::Iter<'a, T>;
pub type GridIterMut<'a, T> = slice::IterMut<'a, T>;
pub type GridChunks<'a, T> = slice::Chunks<'a, T>;
pub type GridChunksMut<'a, T> = slice::ChunksMut<'a, T>;

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Grid<T> {
    pub(crate) cells: Vec<T>,
    pub(crate) dimensions: UVec2,
}

// Grid Layer
impl<T> GridLayer<T> for Grid<T> {
    type MutableReturn<'a> = &'a mut T where T: 'a, Self: 'a;

    #[inline(always)]
    fn new(dimensions: UVec2, cells: Vec<T>) -> Self {
        let count = dimensions.size();
        if cells.len() != count {
            panic!("Dimensions({}) do not match cells({})", count, cells.len());
        }

        Self { cells, dimensions }
    }

    #[inline(always)]
    fn new_clone(dimensions: UVec2, value: T) -> Self
    where
        T: Clone,
    {
        let count = dimensions.size();
        let mut cells = Vec::with_capacity(count);
        cells.resize(count, value);
        Self { cells, dimensions }
    }

    #[inline(always)]
    fn blit_clone(&mut self, to: UVec2, source: &Self, from: UVec2, dimensions: UVec2)
    where
        T: Clone,
    {
        for y in 0..dimensions.y {
            for x in 0..dimensions.x {
                if let Some(val) = source.get([x + from.x, y + from.y].as_uvec2()) {
                    self.set([x + to.x, y + to.y].as_uvec2(), val.clone());
                }
            }
        }
    }

    #[inline(always)]
    fn new_copy(dimensions: UVec2, value: T) -> Self
    where
        T: Copy,
    {
        let count = dimensions.size();
        let mut cells = Vec::with_capacity(count);
        cells.resize_with(count, || value);
        Self { cells, dimensions }
    }

    #[inline(always)]
    fn blit_copy(&mut self, to: UVec2, source: &Self, from: UVec2, dimensions: UVec2)
    where
        T: Copy,
    {
        for y in 0..dimensions.y {
            for x in 0..dimensions.x {
                if let Some(val) = source.get([x + from.x, y + from.y].as_uvec2()) {
                    self.set([x + to.x, y + to.y].as_uvec2(), *val);
                }
            }
        }
    }

    #[inline(always)]
    fn new_default(dimensions: UVec2) -> Self
    where
        T: Default,
    {
        let count = dimensions.size();
        let mut cells = Vec::new();
        cells.resize_with(count, Default::default);
        Self { cells, dimensions }
    }

    #[inline(always)]
    fn new_fn(dimensions: UVec2, f: impl Fn((usize, IVec2)) -> T) -> Self {
        let count = dimensions.size();
        let mut cells = Vec::with_capacity(count);
        dimensions
            .iter()
            .enumerate()
            .for_each(|coord| cells.push(f(coord)));
        Self { cells, dimensions }
    }

    #[inline]
    fn width(&self) -> u32 {
        self.dimensions.x
    }

    #[inline]
    fn height(&self) -> u32 {
        self.dimensions.y
    }

    #[inline]
    fn take(&mut self) -> Vec<T> {
        std::mem::take(&mut self.cells)
    }

    #[inline]
    fn dimensions(&self) -> UVec2 {
        self.dimensions
    }

    #[inline]
    fn len(&self) -> usize {
        self.cells.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    #[inline]
    fn get(&self, index: UVec2) -> Option<&T> {
        self.get_idx(index).map(|idx| &self.cells[idx])
    }

    #[inline]
    fn get_mut(&mut self, index: UVec2) -> Option<&mut T> {
        self.get_idx(index).map(move |idx| &mut self.cells[idx])
    }

    #[inline]
    fn get_unchecked(&self, index: UVec2) -> &T {
        self.cells.index(self.get_idx_unchecked(index))
    }

    #[inline]
    fn get_mut_unchecked(&mut self, index: UVec2) -> &mut T {
        self.cells.index_mut(self.get_idx_unchecked(index))
    }

    #[inline]
    fn set(&mut self, index: UVec2, value: T) -> Option<T> {
        if index.is_valid(self.dimensions) {
            let index = self.get_idx_unchecked(index);
            Some(std::mem::replace(&mut self.cells[index], value))
        } else {
            None
        }
    }

    #[inline]
    fn set_unchecked(&mut self, index: UVec2, value: T) -> T {
        let index = self.get_idx_unchecked(index);
        std::mem::replace(&mut self.cells[index], value)
    }
}

impl<T> GridIterable<T> for Grid<T> {
    type IterChunkMutReturn<'a> = GridChunksMut<'a, T> where T: 'a, Self: 'a;
    type IterChunkReturn<'a> = GridChunks<'a, T> where T: 'a, Self: 'a;
    type IterMutReturn<'a> = GridIterMut<'a, T> where T: 'a, Self: 'a;
    type IterReturn<'a> = GridIter<'a, T> where T: 'a, Self: 'a;

    #[inline]
    fn iter(&self) -> GridIter<T> {
        self.cells.iter()
    }

    /// A mutable iterator over all elements in the grid.
    #[inline]
    fn iter_mut(&mut self) -> GridIterMut<T> {
        self.cells.iter_mut()
    }

    #[inline]
    fn point_iter(&self) -> PointIterRowMajor {
        self.dimensions.iter()
    }

    #[inline]
    fn enumerate(&self) -> GridEnumerate<Self::IterReturn<'_>> {
        self.point_iter().zip(self.iter())
    }

    #[inline]
    fn rows(&self) -> Self::IterChunkReturn<'_> {
        self.cells.chunks(self.dimensions.x as usize)
    }

    #[inline]
    fn rows_mut(&mut self) -> Self::IterChunkMutReturn<'_> {
        self.cells.chunks_mut(self.dimensions.x as usize)
    }

    #[inline]
    fn cols(&self) -> Self::IterChunkReturn<'_> {
        self.cells.chunks(self.dimensions.x as usize)
    }

    #[inline]
    fn cols_mut(&mut self) -> Self::IterChunkMutReturn<'_> {
        self.cells.chunks_mut(self.dimensions.x as usize)
    }

    #[inline]
    fn iter_column(&self, x: usize) -> Option<GridIterCol<Self::IterReturn<'_>>> {
        if x < self.dimensions.size() {
            let w = self.width() as usize;
            return Some(self.cells[x..].iter().step_by(w));
        } else {
            None
        }
    }

    #[inline]
    fn iter_column_unchecked(&self, x: usize) -> GridIterCol<Self::IterReturn<'_>> {
        let w = self.width() as usize;
        return self.cells[x..].iter().step_by(w);
    }
}

///////////////////////////////////////////////////////////////////////////
// Deref/DerefMut
///////////////////////////////////////////////////////////////////////////
// Deref
impl<T> std::ops::Deref for Grid<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.cells
    }
}

// DerefMut
impl<T> std::ops::DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cells
    }
}

///////////////////////////////////////////////////////////////////////////
// Indexing
///////////////////////////////////////////////////////////////////////////

impl<T> Index<usize> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        &self.cells[index]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}

impl<I: GridPoint, T> Index<I> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: I) -> &T {
        &self.cells[index.as_index_unchecked(self.dimensions.x)]
    }
}

impl<I: GridPoint, T> IndexMut<I> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.cells[index.as_index_unchecked(self.dimensions.x)]
    }
}

//#########################################################################
// Serialize
//#########################################################################
#[cfg(feature = "serialize")]
impl<T: Serialize> Serialize for Grid<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serde_state = match serde::Serializer::serialize_struct(serializer, "Grid", 2) {
            Ok(val) => val,
            Err(err) => {
                return Err(err);
            }
        };

        match serde::ser::SerializeStruct::serialize_field(
            &mut serde_state,
            "dimensions",
            &self.dimensions,
        ) {
            Ok(val) => val,
            Err(err) => {
                return Err(err);
            }
        };

        match serde::ser::SerializeStruct::serialize_field(&mut serde_state, "cells", &self.cells) {
            Ok(val) => val,
            Err(err) => {
                return Err(err);
            }
        };

        serde::ser::SerializeStruct::end(serde_state)
    }
}

//#########################################################################
// Deserialize
//#########################################################################
#[cfg(feature = "serialize")]
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Grid<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::__private::de::*;

        #[allow(non_camel_case_types)]
        enum __Grid {
            cells,
            dimensions,
            __ignore,
        }

        struct FieldVisitor;
        impl<'de> serde::de::Visitor<'de> for FieldVisitor {
            type Value = __Grid;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> serde::__private::fmt::Result {
                std::fmt::Formatter::write_str(formatter, "field identifier")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    0_u64 => Ok(__Grid::cells),
                    1_u64 => Ok(__Grid::dimensions),
                    _ => Ok(__Grid::__ignore),
                }
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "cells" => Ok(__Grid::cells),
                    "dimensions" => Ok(__Grid::dimensions),
                    _ => Ok(__Grid::__ignore),
                }
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    b"cells" => Ok(__Grid::cells),
                    b"dimensions" => Ok(__Grid::dimensions),
                    _ => Ok(__Grid::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Grid {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(deserializer, FieldVisitor)
            }
        }

        struct Visitor<'de, T>
        where
            T: serde::Deserialize<'de>,
        {
            marker: std::marker::PhantomData<Grid<T>>,
            lifetime: std::marker::PhantomData<&'de ()>,
        }

        impl<'de, T> serde::de::Visitor<'de> for Visitor<'de, T>
        where
            T: serde::Deserialize<'de>,
        {
            type Value = Grid<T>;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> serde::__private::fmt::Result {
                std::fmt::Formatter::write_str(formatter, "struct Grid")
            }

            #[inline]
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let cells = match match serde::de::SeqAccess::next_element::<Vec<T>>(&mut seq) {
                    Ok(val) => val,
                    Err(err) => {
                        return Err(err);
                    }
                } {
                    Some(value) => value,
                    None => {
                        return Err(serde::de::Error::invalid_length(
                            0usize,
                            &"struct Grid with 2 elements",
                        ));
                    }
                };
                let dimensions = match match serde::de::SeqAccess::next_element::<UVec2>(&mut seq) {
                    Ok(val) => val,
                    Err(err) => {
                        return Err(err);
                    }
                } {
                    Some(value) => value,
                    None => {
                        return Err(serde::de::Error::invalid_length(
                            1usize,
                            &"struct Grid with 2 elements",
                        ));
                    }
                };
                Ok(Grid { cells, dimensions })
            }

            #[inline]
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut cells: Option<Vec<T>> = None;
                let mut dimensions: Option<UVec2> = None;
                while let Some(key) = match serde::de::MapAccess::next_key::<__Grid>(&mut map) {
                    Ok(val) => val,
                    Err(err) => {
                        return Err(err);
                    }
                } {
                    match key {
                        __Grid::cells => {
                            if Option::is_some(&cells) {
                                return Err(<A::Error as serde::de::Error>::duplicate_field(
                                    "cells",
                                ));
                            }
                            cells =
                                Some(match serde::de::MapAccess::next_value::<Vec<T>>(&mut map) {
                                    Ok(val) => val,
                                    Err(err) => {
                                        return Err(err);
                                    }
                                });
                        }
                        __Grid::dimensions => {
                            if Option::is_some(&dimensions) {
                                return Err(<A::Error as serde::de::Error>::duplicate_field(
                                    "dimensions",
                                ));
                            }
                            dimensions =
                                Some(match serde::de::MapAccess::next_value::<UVec2>(&mut map) {
                                    Ok(val) => val,
                                    Err(err) => {
                                        return Err(err);
                                    }
                                });
                        }
                        _ => {
                            let _ = match serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(
                                &mut map,
                            ) {
                                Ok(val) => val,
                                Err(err) => {
                                    return Err(err);
                                }
                            };
                        }
                    }
                }
                let cells = match cells {
                    Some(cells) => cells,
                    None => match missing_field("cells") {
                        Ok(val) => val,
                        Err(err) => {
                            return Err(err);
                        }
                    },
                };
                let dimensions = match dimensions {
                    Some(dimensions) => dimensions,
                    None => match missing_field("dimensions") {
                        Ok(val) => val,
                        Err(err) => {
                            return Err(err);
                        }
                    },
                };
                Ok(Grid { cells, dimensions })
            }
        }

        const FIELDS: &[&str] = &["cells", "dimensions"];
        serde::Deserializer::deserialize_struct(
            deserializer,
            "Grid",
            FIELDS,
            Visitor {
                marker: std::marker::PhantomData::<Self>,
                lifetime: std::marker::PhantomData,
            },
        )
    }
}
