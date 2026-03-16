pub struct BitEnumerator {
    data: Vec<u8>,
    index: usize,
    mask: u8,
}

impl BitEnumerator {
    pub fn new(data: Vec<u8>) -> Self { Self { data, index: 0, mask: 0x80 } }

    pub fn has_next(&self) -> bool {
        self.mask != 0 || self.index != self.data.len() - 1
    }

    pub fn next(&mut self) -> bool {
        assert!(self.has_next(), "BitEnumerator underflow");

        if self.mask == 0 {
            self.mask = 0x80;
            self.index += 1;
        }

        let b = (self.data[self.index] & self.mask) != 0;
        self.mask >>= 1;
        b
    }

    pub fn next_uint2(&mut self) -> u32 {
        let mut bit_mask: u32 = 0x02;
        let mut value: u32 = 0;
        for _ in 0..2 {
            if self.next() {
                value |= bit_mask;
            }
            bit_mask >>= 1;
        }
        value
    }

    pub fn next_uint8(&mut self) -> u32 {
        let mut bit_mask: u32 = 0x80;
        let mut value: u32 = 0;
        for _ in 0..8 {
            if self.next() {
                value |= bit_mask;
            }
            bit_mask >>= 1;
        }
        value
    }

    #[allow(dead_code)]
    pub fn next_uint16(&mut self) -> u32 {
        let mut bit_mask: u32 = 0x8000;
        let mut value: u32 = 0;
        for _ in 0..16 {
            if self.next() {
                value |= bit_mask;
            }
            bit_mask >>= 1;
        }
        value
    }

    pub fn next_frac(&mut self) -> f64 { self.next_uint16() as f64 / 65535.0 }

    pub fn for_all<F: FnMut(bool)>(&mut self, mut f: F) {
        while self.has_next() {
            f(self.next());
        }
    }
}

pub struct BitAggregator {
    data: Vec<u8>,
    bit_mask: u8,
}

impl BitAggregator {
    pub fn new() -> Self { Self { data: Vec::new(), bit_mask: 0 } }

    pub fn append(&mut self, bit: bool) {
        if self.bit_mask == 0 {
            self.bit_mask = 0x80;
            self.data.push(0);
        }

        if bit {
            let last = self.data.len() - 1;
            self.data[last] |= self.bit_mask;
        }

        self.bit_mask >>= 1;
    }

    pub fn data(&self) -> Vec<u8> { self.data.clone() }
}
