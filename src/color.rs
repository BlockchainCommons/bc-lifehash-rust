#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Default for Color {
    fn default() -> Self { Self::BLACK }
}

// Match C++ numeric helpers exactly.
// C++ uses fmodf (32-bit), sqrtf (32-bit), powf (32-bit) for specific
// operations.

#[inline]
pub fn clamped(n: f64) -> f64 { n.clamp(0.0, 1.0) }

/// C++ uses fmodf (f32 precision) even though arguments are f64.
#[inline]
pub fn modulo(dividend: f64, divisor: f64) -> f64 {
    let a = (dividend as f32) % (divisor as f32);
    let b = (a + divisor as f32) % (divisor as f32);
    b as f64
}

#[inline]
pub fn lerp_to(to_a: f64, to_b: f64, t: f64) -> f64 { t * (to_b - to_a) + to_a }

#[inline]
pub fn lerp_from(from_a: f64, from_b: f64, t: f64) -> f64 {
    (from_a - t) / (from_a - from_b)
}

#[inline]
pub fn lerp(from_a: f64, from_b: f64, to_c: f64, to_d: f64, t: f64) -> f64 {
    lerp_to(to_c, to_d, lerp_from(from_a, from_b, t))
}

impl Color {
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0 };
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0 };
    #[allow(dead_code)]
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0 };
    #[allow(dead_code)]
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0 };
    #[allow(dead_code)]
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0 };
    #[allow(dead_code)]
    pub const CYAN: Color = Color { r: 0.0, g: 1.0, b: 1.0 };
    #[allow(dead_code)]
    pub const MAGENTA: Color = Color { r: 1.0, g: 0.0, b: 1.0 };
    #[allow(dead_code)]
    pub const YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0 };

    pub fn new(r: f64, g: f64, b: f64) -> Self { Self { r, g, b } }

    pub fn from_uint8_values(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f64 / 255.0,
            g: g as f64 / 255.0,
            b: b as f64 / 255.0,
        }
    }

    pub fn lerp_to(&self, other: &Color, t: f64) -> Color {
        let f = clamped(t);
        let red = clamped(self.r * (1.0 - f) + other.r * f);
        let green = clamped(self.g * (1.0 - f) + other.g * f);
        let blue = clamped(self.b * (1.0 - f) + other.b * f);
        Color::new(red, green, blue)
    }

    pub fn lighten(&self, t: f64) -> Color { self.lerp_to(&Color::WHITE, t) }

    pub fn darken(&self, t: f64) -> Color { self.lerp_to(&Color::BLACK, t) }

    pub fn burn(&self, t: f64) -> Color {
        let f = (1.0 - t).max(1.0e-7);
        Color::new(
            (1.0 - (1.0 - self.r) / f).min(1.0),
            (1.0 - (1.0 - self.g) / f).min(1.0),
            (1.0 - (1.0 - self.b) / f).min(1.0),
        )
    }

    /// Luminance using C++ sqrtf/powf (f32 precision).
    pub fn luminance(&self) -> f64 {
        let r = (0.299 * self.r) as f32;
        let g = (0.587 * self.g) as f32;
        let b = (0.114 * self.b) as f32;
        let val = r.powi(2) + g.powi(2) + b.powi(2);
        val.sqrt() as f64
    }
}
