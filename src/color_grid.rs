use crate::{
    color::Color, color_func::ColorFunc, frac_grid::FracGrid, grid::Grid,
    patterns::Pattern,
};

struct Transform {
    transpose: bool,
    reflect_x: bool,
    reflect_y: bool,
}

pub struct ColorGrid {
    pub grid: Grid<Color>,
}

impl ColorGrid {
    pub fn new(
        frac_grid: &FracGrid,
        gradient: &ColorFunc,
        pattern: Pattern,
    ) -> Self {
        let multiplier = if pattern == Pattern::Fiducial { 1 } else { 2 };
        let target_width = frac_grid.grid.width * multiplier;
        let target_height = frac_grid.grid.height * multiplier;

        let mut grid: Grid<Color> = Grid::new(target_width, target_height);
        let max_x = target_width - 1;
        let max_y = target_height - 1;

        let transforms: Vec<Transform> = match pattern {
            Pattern::Snowflake => vec![
                Transform {
                    transpose: false,
                    reflect_x: false,
                    reflect_y: false,
                },
                Transform {
                    transpose: false,
                    reflect_x: true,
                    reflect_y: false,
                },
                Transform {
                    transpose: false,
                    reflect_x: false,
                    reflect_y: true,
                },
                Transform {
                    transpose: false,
                    reflect_x: true,
                    reflect_y: true,
                },
            ],
            Pattern::Pinwheel => vec![
                Transform {
                    transpose: false,
                    reflect_x: false,
                    reflect_y: false,
                },
                Transform {
                    transpose: true,
                    reflect_x: true,
                    reflect_y: false,
                },
                Transform {
                    transpose: true,
                    reflect_x: false,
                    reflect_y: true,
                },
                Transform {
                    transpose: false,
                    reflect_x: true,
                    reflect_y: true,
                },
            ],
            Pattern::Fiducial => vec![Transform {
                transpose: false,
                reflect_x: false,
                reflect_y: false,
            }],
        };

        let frac_width = frac_grid.grid.width;
        let frac_height = frac_grid.grid.height;
        for y in 0..frac_height {
            for x in 0..frac_width {
                let value = frac_grid.grid.get_value(x, y);
                let color = gradient(value);
                for t in &transforms {
                    let mut px = x;
                    let mut py = y;
                    if t.transpose {
                        std::mem::swap(&mut px, &mut py);
                    }
                    if t.reflect_x {
                        px = max_x - px;
                    }
                    if t.reflect_y {
                        py = max_y - py;
                    }
                    grid.set_value(color, px, py);
                }
            }
        }

        Self { grid }
    }

    pub fn colors(&self) -> Vec<f64> {
        let mut result = Vec::with_capacity(self.grid.storage.len() * 3);
        for c in &self.grid.storage {
            result.push(c.r);
            result.push(c.g);
            result.push(c.b);
        }
        result
    }
}
