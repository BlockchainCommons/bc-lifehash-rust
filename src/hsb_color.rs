use crate::color::{Color, clamped, modulo};

pub struct HSBColor {
    pub hue: f64,
    pub saturation: f64,
    pub brightness: f64,
}

impl HSBColor {
    #[allow(dead_code)]
    pub fn new(hue: f64, saturation: f64, brightness: f64) -> Self {
        Self { hue, saturation, brightness }
    }

    pub fn from_hue(hue: f64) -> Self {
        Self { hue, saturation: 1.0, brightness: 1.0 }
    }

    pub fn color(&self) -> Color {
        let v = clamped(self.brightness);
        let s = clamped(self.saturation);

        if s <= 0.0 {
            return Color::new(v, v, v);
        }

        let mut h = modulo(self.hue, 1.0);
        if h < 0.0 {
            h += 1.0;
        }
        h *= 6.0;
        // C++ uses floorf (f32 precision)
        let i = (h as f32).floor() as i32;
        let f = h - i as f64;
        let p = v * (1.0 - s);
        let q = v * (1.0 - s * f);
        let t = v * (1.0 - s * (1.0 - f));

        match i {
            0 => Color::new(v, t, p),
            1 => Color::new(q, v, p),
            2 => Color::new(p, v, t),
            3 => Color::new(p, q, v),
            4 => Color::new(t, p, v),
            5 => Color::new(v, p, q),
            _ => panic!("Internal error in HSB conversion"),
        }
    }
}
