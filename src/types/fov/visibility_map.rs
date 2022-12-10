use crate::prelude::*;

#[derive(Default, Debug, Clone)]
pub struct VisibilityMap<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> {
    visible_positions: HashSet<Position<GRID_WIDTH, GRID_HEIGHT>>,
}

impl<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> VisibilityMap<GRID_WIDTH, GRID_HEIGHT> {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            visible_positions: HashSet::new(),
        }
    }

    pub fn iter(&self) -> bevy::utils::hashbrown::hash_set::Iter<Position<GRID_WIDTH, GRID_HEIGHT>> {
        self.visible_positions.iter()
    }
}

impl<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> FovReceiver<GRID_WIDTH, GRID_HEIGHT>
    for VisibilityMap<GRID_WIDTH, GRID_HEIGHT>
{
    fn get_visible(&self, position: Position<GRID_WIDTH, GRID_HEIGHT>) -> bool {
        self.visible_positions.contains(&position)
    }

    fn set_visible(&mut self, position: Position<GRID_WIDTH, GRID_HEIGHT>) {
        self.visible_positions.insert(position);
    }

    fn get_all(&self) -> HashSet<Position<GRID_WIDTH, GRID_HEIGHT>> { self.visible_positions.clone() }
}

impl<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> IntoIterator for VisibilityMap<GRID_WIDTH, GRID_HEIGHT> {
    type IntoIter = bevy::utils::hashbrown::hash_set::IntoIter<Position<GRID_WIDTH, GRID_HEIGHT>>;
    type Item = Position<GRID_WIDTH, GRID_HEIGHT>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter { self.visible_positions.into_iter() }
}
