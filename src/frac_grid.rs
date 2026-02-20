use crate::cell_grid::CellGrid;
use crate::grid::Grid;

pub struct FracGrid {
    pub grid: Grid<f64>,
}

impl FracGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
        }
    }

    pub fn overlay(&mut self, cell_grid: &CellGrid, frac: f64) {
        let width = self.grid.width;
        let height = self.grid.height;
        for y in 0..height {
            for x in 0..width {
                if cell_grid.grid.get_value(x, y) {
                    self.grid.set_value(frac, x, y);
                }
            }
        }
    }
}
