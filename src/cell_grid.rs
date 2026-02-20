use crate::bit_enumerator::{BitAggregator, BitEnumerator};
use crate::change_grid::ChangeGrid;
use crate::grid::Grid;

pub struct CellGrid {
    pub grid: Grid<bool>,
}

impl CellGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
        }
    }

    #[inline]
    fn is_alive_in_next_generation(current_alive: bool, neighbors_count: usize) -> bool {
        if current_alive {
            neighbors_count == 2 || neighbors_count == 3
        } else {
            neighbors_count == 3
        }
    }

    fn count_neighbors(&self, px: usize, py: usize) -> usize {
        let mut total = 0;
        self.grid.for_neighborhood(px, py, |ox, oy, nx, ny| {
            if ox == 0 && oy == 0 {
                return;
            }
            if self.grid.get_value(nx, ny) {
                total += 1;
            }
        });
        total
    }

    pub fn data(&self) -> Vec<u8> {
        let mut a = BitAggregator::new();
        self.grid.for_all(|x, y| {
            a.append(self.grid.get_value(x, y));
        });
        a.data()
    }

    pub fn set_data(&mut self, data: &[u8]) {
        debug_assert_eq!(self.grid.width * self.grid.height, data.len() * 8);
        let mut e = BitEnumerator::new(data.to_vec());
        let mut i = 0;
        e.for_all(|b| {
            self.grid.storage[i] = b;
            i += 1;
        });
        debug_assert_eq!(i, self.grid.storage.len());
    }

    pub fn next_generation(
        &self,
        current_change_grid: &ChangeGrid,
        next_cell_grid: &mut CellGrid,
        next_change_grid: &mut ChangeGrid,
    ) {
        next_cell_grid.grid.set_all(false);
        next_change_grid.grid.set_all(false);
        let width = self.grid.width;
        let height = self.grid.height;
        for y in 0..height {
            for x in 0..width {
                let current_alive = self.grid.get_value(x, y);
                if current_change_grid.grid.get_value(x, y) {
                    let neighbors_count = self.count_neighbors(x, y);
                    let next_alive = Self::is_alive_in_next_generation(current_alive, neighbors_count);
                    if next_alive {
                        next_cell_grid.grid.set_value(true, x, y);
                    }
                    if current_alive != next_alive {
                        next_change_grid.set_changed(x, y);
                    }
                } else {
                    next_cell_grid.grid.set_value(current_alive, x, y);
                }
            }
        }
    }
}
