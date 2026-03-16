use crate::color::{Color, modulo};

pub type ColorFunc = Box<dyn Fn(f64) -> Color>;

pub fn reverse(c: ColorFunc) -> ColorFunc { Box::new(move |t| c(1.0 - t)) }

pub fn blend2(color1: Color, color2: Color) -> ColorFunc {
    Box::new(move |t| color1.lerp_to(&color2, t))
}

pub fn blend(colors: Vec<Color>) -> ColorFunc {
    let count = colors.len();
    match count {
        0 => blend2(Color::BLACK, Color::BLACK),
        1 => blend2(colors[0], colors[0]),
        2 => blend2(colors[0], colors[1]),
        _ => Box::new(move |t| {
            if t >= 1.0 {
                return colors[count - 1];
            } else if t <= 0.0 {
                return colors[0];
            }
            let segments = count - 1;
            let s = t * segments as f64;
            let segment = s as usize;
            let segment_frac = modulo(s, 1.0);
            let c1 = colors[segment];
            let c2 = colors[segment + 1];
            c1.lerp_to(&c2, segment_frac)
        }),
    }
}
