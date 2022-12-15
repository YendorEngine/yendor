use std::{
    ops::{Index, IndexMut},
    slice,
};

use crate::prelude::*;
pub type GridIter<'a, T> = slice::Iter<'a, T>;
pub type GridIterMut<'a, T> = slice::IterMut<'a, T>;
pub type GridChunks<'a, T> = slice::Chunks<'a, T>;
pub type GridChunksMut<'a, T> = slice::ChunksMut<'a, T>;

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Grid<T> {
    pub cells: Vec<T>,
    pub dimensions: UVec2,
}

// Grid Layer
impl<T> GridLayer<T> for Grid<T> {
    type MutableReturn<'a> = &'a mut T where T: 'a, Self: 'a;

    #[inline(always)]
    fn new_clone(dimensions: impl Dimensions, value: T) -> Self
    where T: Clone {
        let count = dimensions.size();
        let mut cells = Vec::with_capacity(count);
        cells.resize(count, value);
        Self {
            cells,
            dimensions: dimensions.as_uvec2(),
        }
    }

    #[inline(always)]
    fn blit_clone(&mut self, to: impl Point, source: &Self, from: impl Point, size: impl Dimensions)
    where T: Clone {
        for y in 0..size.height() {
            for x in 0..size.width() {
                if let Some(val) = source.get((x + from.x_uint32(), y + from.y_uint32())) {
                    self.set((x + to.x_uint32(), y + to.y_uint32()), val.clone());
                }
            }
        }
    }

    #[inline(always)]
    fn new_copy(size: impl Dimensions, value: T) -> Self
    where T: Copy {
        let count = size.size();
        let mut cells = Vec::with_capacity(count);
        cells.resize_with(count, || value);
        Self {
            cells,
            dimensions: size.as_uvec2(),
        }
    }

    #[inline(always)]
    fn blit_copy(&mut self, to: impl Point, source: &Self, from: impl Point, size: impl Dimensions)
    where T: Copy {
        for y in 0..size.height() {
            for x in 0..size.width() {
                if let Some(val) = source.get((x + from.x_uint32(), y + from.y_uint32())) {
                    self.set((x + to.x_uint32(), y + to.y_uint32()), *val);
                }
            }
        }
    }

    #[inline(always)]
    fn new_default(size: impl Dimensions) -> Self
    where T: Default {
        let count = size.size();
        let mut cells = Vec::new();
        cells.resize_with(count, Default::default);
        Self {
            cells,
            dimensions: size.as_uvec2(),
        }
    }

    #[inline(always)]
    fn new_fn(size: impl Dimensions, f: impl Fn(IVec2) -> T) -> Self {
        let count = size.size();
        let mut cells = Vec::with_capacity(count);
        for coord in size.iter() {
            cells.push(f(coord));
        }
        Self {
            cells,
            dimensions: size.as_uvec2(),
        }
    }

    #[inline]
    fn width(&self) -> u32 { self.dimensions.width() }

    #[inline]
    fn height(&self) -> u32 { self.dimensions.height() }

    #[inline]
    fn dimensions(&self) -> UVec2 { self.dimensions }

    #[inline]
    fn len(&self) -> usize { self.cells.len() }

    #[inline]
    fn is_empty(&self) -> bool { self.cells.is_empty() }

    #[inline]
    fn in_bounds(&self, pos: impl Point) -> bool { pos.is_valid(self.dimensions) }

    #[inline]
    fn get_idx(&self, pos: impl Point) -> Option<usize> {
        if pos.is_valid(self.dimensions) { Some(self.get_idx_unchecked(pos)) } else { None }
    }

    #[inline]
    fn get_idx_unchecked(&self, point: impl Point) -> usize { point.as_index_unchecked(self.width()) }

    #[inline]
    fn index_to_pt(&self, idx: usize) -> Option<IVec2> {
        let pt = self.index_to_pt_unchecked(idx);
        if pt.is_valid(self.dimensions) { Some(pt) } else { None }
    }

    #[inline]
    fn index_to_pt_unchecked(&self, idx: usize) -> IVec2 {
        let x = idx % self.width() as usize;
        let y = idx / self.width() as usize;
        IVec2::new(x as i32, y as i32)
    }

    #[inline]
    fn get(&self, index: impl Point) -> Option<&T> { self.get_idx(index).map(|idx| &self.cells[idx]) }

    #[inline]
    fn get_mut(&mut self, index: impl Point) -> Option<&mut T> {
        self.get_idx(index).map(move |idx| &mut self.cells[idx])
    }

    #[inline]
    fn get_unchecked(&self, index: impl Point) -> &T { self.cells.index(self.get_idx_unchecked(index)) }

    #[inline]
    fn get_mut_unchecked(&mut self, index: impl Point) -> &mut T {
        self.cells.index_mut(self.get_idx_unchecked(index))
    }

    #[inline]
    fn set(&mut self, index: impl Point, value: T) -> Option<T> {
        if index.is_valid(self.dimensions) {
            let index = self.get_idx_unchecked(index);
            Some(std::mem::replace(&mut self.cells[index], value))
        } else {
            None
        }
    }

    #[inline]
    fn set_unchecked(&mut self, index: impl Point, value: T) -> T {
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
    fn iter(&self) -> GridIter<T> { self.cells.iter() }

    /// A mutable iterator over all elements in the grid.
    #[inline]
    fn iter_mut(&mut self) -> GridIterMut<T> { self.cells.iter_mut() }

    #[inline]
    fn point_iter(&self) -> PointIterRowMajor { self.dimensions.iter() }

    #[inline]
    fn enumerate(&self) -> GridEnumerate<Self::IterReturn<'_>> { self.point_iter().zip(self.iter()) }

    #[inline]
    fn rows(&self) -> Self::IterChunkReturn<'_> { self.cells.chunks(self.dimensions.width() as usize) }

    #[inline]
    fn rows_mut(&mut self) -> Self::IterChunkMutReturn<'_> {
        self.cells.chunks_mut(self.dimensions.width() as usize)
    }

    #[inline]
    fn cols(&self) -> Self::IterChunkReturn<'_> { self.cells.chunks(self.dimensions.width() as usize) }

    #[inline]
    fn cols_mut(&mut self) -> Self::IterChunkMutReturn<'_> {
        self.cells.chunks_mut(self.dimensions.width() as usize)
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

    fn deref(&self) -> &Self::Target { &self.cells }
}

// DerefMut
impl<T> std::ops::DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.cells }
}

///////////////////////////////////////////////////////////////////////////
// Indexing
///////////////////////////////////////////////////////////////////////////

impl<T> Index<usize> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T { &self.cells[index] }
}

impl<T> std::ops::IndexMut<usize> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.cells[index] }
}

//#########################################################################
// Serialize
//#########################################################################
#[cfg(feature = "serialize")]
impl<T: Serialize> Serialize for Grid<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let mut serde_state = match serde::Serializer::serialize_struct(serializer, "Grid", 2) {
            Ok(val) => val,
            Err(err) => {
                return Err(err);
            },
        };

        match serde::ser::SerializeStruct::serialize_field(&mut serde_state, "dimensions", &self.dimensions) {
            Ok(val) => val,
            Err(err) => {
                return Err(err);
            },
        };

        match serde::ser::SerializeStruct::serialize_field(&mut serde_state, "cells", &self.cells) {
            Ok(val) => val,
            Err(err) => {
                return Err(err);
            },
        };

        serde::ser::SerializeStruct::end(serde_state)
    }
}

//#########################################################################
// Deserialize
//#########################################################################
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Grid<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        use serde::__private::de::*;

        #[allow(non_camel_case_types)]
        enum Field {
            cells,
            dimensions,
            __ignore,
        }

        struct FieldVisitor;
        impl<'de> serde::de::Visitor<'de> for FieldVisitor {
            type Value = Field;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> serde::__private::fmt::Result {
                std::fmt::Formatter::write_str(formatter, "field identifier")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where E: serde::de::Error {
                match value {
                    0_u64 => Ok(Field::cells),
                    1_u64 => Ok(Field::dimensions),
                    _ => Ok(Field::__ignore),
                }
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where E: serde::de::Error {
                match value {
                    "cells" => Ok(Field::cells),
                    "dimensions" => Ok(Field::dimensions),
                    _ => Ok(Field::__ignore),
                }
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
            where E: serde::de::Error {
                match value {
                    b"cells" => Ok(Field::cells),
                    b"dimensions" => Ok(Field::dimensions),
                    _ => Ok(Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for Field {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: serde::Deserializer<'de> {
                serde::Deserializer::deserialize_identifier(deserializer, FieldVisitor)
            }
        }

        struct Visitor<'de, T>
        where T: serde::Deserialize<'de>
        {
            marker: std::marker::PhantomData<Grid<T>>,
            lifetime: std::marker::PhantomData<&'de ()>,
        }

        impl<'de, T> serde::de::Visitor<'de> for Visitor<'de, T>
        where T: serde::Deserialize<'de>
        {
            type Value = Grid<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> serde::__private::fmt::Result {
                std::fmt::Formatter::write_str(formatter, "struct Grid")
            }

            #[inline]
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where A: serde::de::SeqAccess<'de> {
                let cells = match match serde::de::SeqAccess::next_element::<Vec<T>>(&mut seq) {
                    Ok(val) => val,
                    Err(err) => {
                        return Err(err);
                    },
                } {
                    Some(value) => value,
                    None => {
                        return Err(serde::de::Error::invalid_length(
                            0usize,
                            &"struct Grid with 2 elements",
                        ));
                    },
                };
                let dimensions = match match serde::de::SeqAccess::next_element::<UVec2>(&mut seq) {
                    Ok(val) => val,
                    Err(err) => {
                        return Err(err);
                    },
                } {
                    Some(value) => value,
                    None => {
                        return Err(serde::de::Error::invalid_length(
                            1usize,
                            &"struct Grid with 2 elements",
                        ));
                    },
                };
                Ok(Grid { cells, dimensions })
            }

            #[inline]
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where A: serde::de::MapAccess<'de> {
                let mut cells: Option<Vec<T>> = None;
                let mut dimensions: Option<UVec2> = None;
                while let Some(key) = match serde::de::MapAccess::next_key::<Field>(&mut map) {
                    Ok(val) => val,
                    Err(err) => {
                        return Err(err);
                    },
                } {
                    match key {
                        Field::cells => {
                            if Option::is_some(&cells) {
                                return Err(<A::Error as serde::de::Error>::duplicate_field("cells"));
                            }
                            cells = Some(match serde::de::MapAccess::next_value::<Vec<T>>(&mut map) {
                                Ok(val) => val,
                                Err(err) => {
                                    return Err(err);
                                },
                            });
                        },
                        Field::dimensions => {
                            if Option::is_some(&dimensions) {
                                return Err(<A::Error as serde::de::Error>::duplicate_field(
                                    "dimensions",
                                ));
                            }
                            dimensions = Some(match serde::de::MapAccess::next_value::<UVec2>(&mut map) {
                                Ok(val) => val,
                                Err(err) => {
                                    return Err(err);
                                },
                            });
                        },
                        _ => {
                            let _ = match serde::de::MapAccess::next_value::<serde::de::IgnoredAny>(&mut map)
                            {
                                Ok(val) => val,
                                Err(err) => {
                                    return Err(err);
                                },
                            };
                        },
                    }
                }
                let cells = match cells {
                    Some(cells) => cells,
                    None => match missing_field("cells") {
                        Ok(val) => val,
                        Err(err) => {
                            return Err(err);
                        },
                    },
                };
                let dimensions = match dimensions {
                    Some(dimensions) => dimensions,
                    None => match missing_field("dimensions") {
                        Ok(val) => val,
                        Err(err) => {
                            return Err(err);
                        },
                    },
                };
                Ok(Grid { cells, dimensions })
            }
        }

        const FIELDS: &[&str] = &["cells", "dimensions"];
        serde::Deserializer::deserialize_struct(deserializer, "Grid", FIELDS, Visitor {
            marker: std::marker::PhantomData::<Grid<T>>,
            lifetime: std::marker::PhantomData,
        })
    }
}
