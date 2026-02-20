pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub storage: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let capacity = width * height;
        Self {
            width,
            height,
            storage: vec![T::default(); capacity],
        }
    }

    #[inline]
    fn offset(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    #[inline]
    fn circular_index(index: i32, modulus: i32) -> usize {
        ((index % modulus + modulus) % modulus) as usize
    }

    pub fn set_all(&mut self, value: T) {
        self.storage.fill(value);
    }

    #[inline]
    pub fn set_value(&mut self, value: T, x: usize, y: usize) {
        let off = self.offset(x, y);
        self.storage[off] = value;
    }

    #[inline]
    pub fn get_value(&self, x: usize, y: usize) -> T {
        let off = self.offset(x, y);
        self.storage[off].clone()
    }

    pub fn for_all<F: FnMut(usize, usize)>(&self, mut f: F) {
        for y in 0..self.height {
            for x in 0..self.width {
                f(x, y);
            }
        }
    }

    pub fn for_neighborhood<F: FnMut(i32, i32, usize, usize)>(&self, px: usize, py: usize, mut f: F) {
        for oy in -1..=1_i32 {
            for ox in -1..=1_i32 {
                let nx = Self::circular_index(ox + px as i32, self.width as i32);
                let ny = Self::circular_index(oy + py as i32, self.height as i32);
                f(ox, oy, nx, ny);
            }
        }
    }
}
