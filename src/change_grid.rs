use crate::grid::Grid;

pub struct ChangeGrid {
    pub grid: Grid<bool>,
}

impl ChangeGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self { grid: Grid::new(width, height) }
    }

    pub fn set_changed(&mut self, px: usize, py: usize) {
        let width = self.grid.width;
        let height = self.grid.height;
        for oy in -1..=1_i32 {
            for ox in -1..=1_i32 {
                let nx = ((ox + px as i32) % width as i32 + width as i32)
                    as usize
                    % width;
                let ny = ((oy + py as i32) % height as i32 + height as i32)
                    as usize
                    % height;
                self.grid.set_value(true, nx, ny);
            }
        }
    }
}
