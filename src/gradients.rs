use crate::Version;
use crate::bit_enumerator::BitEnumerator;
use crate::color::{Color, lerp, modulo};
use crate::color_func::{ColorFunc, blend, blend2, reverse};
use crate::hsb_color::HSBColor;

fn grayscale() -> ColorFunc {
    blend2(Color::BLACK, Color::WHITE)
}

fn select_grayscale(entropy: &mut BitEnumerator) -> ColorFunc {
    if entropy.next() {
        grayscale()
    } else {
        reverse(grayscale())
    }
}

fn make_hue(t: f64) -> Color {
    HSBColor::from_hue(t).color()
}

fn spectrum() -> ColorFunc {
    blend(vec![
        Color::from_uint8_values(0, 168, 222),
        Color::from_uint8_values(51, 51, 145),
        Color::from_uint8_values(233, 19, 136),
        Color::from_uint8_values(235, 45, 46),
        Color::from_uint8_values(253, 233, 43),
        Color::from_uint8_values(0, 158, 84),
        Color::from_uint8_values(0, 168, 222),
    ])
}

fn spectrum_cmyk_safe() -> ColorFunc {
    blend(vec![
        Color::from_uint8_values(0, 168, 222),
        Color::from_uint8_values(41, 60, 130),
        Color::from_uint8_values(210, 59, 130),
        Color::from_uint8_values(217, 63, 53),
        Color::from_uint8_values(244, 228, 81),
        Color::from_uint8_values(0, 158, 84),
        Color::from_uint8_values(0, 168, 222),
    ])
}

fn adjust_for_luminance(color: &Color, contrast_color: &Color) -> Color {
    let lum = color.luminance();
    let contrast_lum = contrast_color.luminance();
    let threshold = 0.6;
    let offset = (lum - contrast_lum).abs();
    if offset > threshold {
        return *color;
    }
    let boost = 0.7;
    let t = lerp(0.0, threshold, boost, 0.0, offset);
    if contrast_lum > lum {
        color.darken(t).burn(t * 0.6)
    } else {
        color.lighten(t).burn(t * 0.6)
    }
}

fn monochromatic(entropy: &mut BitEnumerator, hue_generator: &ColorFunc) -> ColorFunc {
    let hue = entropy.next_frac();
    let is_tint = entropy.next();
    let is_reversed = entropy.next();
    let key_advance = entropy.next_frac() * 0.3 + 0.05;
    let neutral_advance = entropy.next_frac() * 0.3 + 0.05;

    let mut key_color = hue_generator(hue);

    let contrast_brightness;
    if is_tint {
        contrast_brightness = 1.0;
        key_color = key_color.darken(0.5);
    } else {
        contrast_brightness = 0.0;
    }
    let gs = grayscale();
    let neutral_color = gs(contrast_brightness);

    let key_color_2 = key_color.lerp_to(&neutral_color, key_advance);
    let neutral_color_2 = neutral_color.lerp_to(&key_color, neutral_advance);

    let gradient = blend2(key_color_2, neutral_color_2);
    if is_reversed { reverse(gradient) } else { gradient }
}

fn monochromatic_fiducial(entropy: &mut BitEnumerator) -> ColorFunc {
    let hue = entropy.next_frac();
    let is_reversed = entropy.next();
    let is_tint = entropy.next();

    let contrast_color = if is_tint { Color::WHITE } else { Color::BLACK };
    let spec = spectrum_cmyk_safe();
    let key_color = adjust_for_luminance(&spec(hue), &contrast_color);

    let gradient = blend(vec![key_color, contrast_color, key_color]);
    if is_reversed { reverse(gradient) } else { gradient }
}

fn complementary(entropy: &mut BitEnumerator, hue_generator: &ColorFunc) -> ColorFunc {
    let spectrum1 = entropy.next_frac();
    let spectrum2 = modulo(spectrum1 + 0.5, 1.0);
    let lighter_advance = entropy.next_frac() * 0.3;
    let darker_advance = entropy.next_frac() * 0.3;
    let is_reversed = entropy.next();

    let color1 = hue_generator(spectrum1);
    let color2 = hue_generator(spectrum2);

    let luma1 = color1.luminance();
    let luma2 = color2.luminance();

    let (darker_color, lighter_color) = if luma1 > luma2 {
        (color2, color1)
    } else {
        (color1, color2)
    };

    let adjusted_lighter = lighter_color.lighten(lighter_advance);
    let adjusted_darker = darker_color.darken(darker_advance);

    let gradient = blend2(adjusted_darker, adjusted_lighter);
    if is_reversed { reverse(gradient) } else { gradient }
}

fn complementary_fiducial(entropy: &mut BitEnumerator) -> ColorFunc {
    let spectrum1 = entropy.next_frac();
    let spectrum2 = modulo(spectrum1 + 0.5, 1.0);
    let is_tint = entropy.next();
    let is_reversed = entropy.next();
    let neutral_color_bias = entropy.next();

    let neutral_color = if is_tint { Color::WHITE } else { Color::BLACK };
    let spec = spectrum_cmyk_safe();
    let color1 = spec(spectrum1);
    let color2 = spec(spectrum2);

    let bias_color = if neutral_color_bias { color1 } else { color2 };
    let biased_neutral_color = neutral_color.lerp_to(&bias_color, 0.2).burn(0.1);

    let gradient = blend(vec![
        adjust_for_luminance(&color1, &biased_neutral_color),
        biased_neutral_color,
        adjust_for_luminance(&color2, &biased_neutral_color),
    ]);
    if is_reversed { reverse(gradient) } else { gradient }
}

fn triadic(entropy: &mut BitEnumerator, hue_generator: &ColorFunc) -> ColorFunc {
    let spectrum1 = entropy.next_frac();
    let spectrum2 = modulo(spectrum1 + 1.0 / 3.0, 1.0);
    let spectrum3 = modulo(spectrum1 + 2.0 / 3.0, 1.0);
    let lighter_advance = entropy.next_frac() * 0.3;
    let darker_advance = entropy.next_frac() * 0.3;
    let is_reversed = entropy.next();

    let color1 = hue_generator(spectrum1);
    let color2 = hue_generator(spectrum2);
    let color3 = hue_generator(spectrum3);

    let mut colors = [color1, color2, color3];
    colors.sort_by(|a, b| a.luminance().partial_cmp(&b.luminance()).unwrap());

    let darker_color = colors[0];
    let middle_color = colors[1];
    let lighter_color = colors[2];

    let adjusted_lighter = lighter_color.lighten(lighter_advance);
    let adjusted_darker = darker_color.darken(darker_advance);

    let gradient = blend(vec![adjusted_lighter, middle_color, adjusted_darker]);
    if is_reversed { reverse(gradient) } else { gradient }
}

fn triadic_fiducial(entropy: &mut BitEnumerator) -> ColorFunc {
    let spectrum1 = entropy.next_frac();
    let spectrum2 = modulo(spectrum1 + 1.0 / 3.0, 1.0);
    let spectrum3 = modulo(spectrum1 + 2.0 / 3.0, 1.0);
    let is_tint = entropy.next();
    let neutral_insert_index = (entropy.next_uint8() % 2 + 1) as usize;
    let is_reversed = entropy.next();

    let neutral_color = if is_tint { Color::WHITE } else { Color::BLACK };

    let spec = spectrum_cmyk_safe();
    let mut colors = vec![spec(spectrum1), spec(spectrum2), spec(spectrum3)];

    match neutral_insert_index {
        1 => {
            colors[0] = adjust_for_luminance(&colors[0], &neutral_color);
            colors[1] = adjust_for_luminance(&colors[1], &neutral_color);
            colors[2] = adjust_for_luminance(&colors[2], &colors[1]);
        }
        2 => {
            colors[1] = adjust_for_luminance(&colors[1], &neutral_color);
            colors[2] = adjust_for_luminance(&colors[2], &neutral_color);
            colors[0] = adjust_for_luminance(&colors[0], &colors[1]);
        }
        _ => panic!("Internal error"),
    }

    colors.insert(neutral_insert_index, neutral_color);

    let gradient = blend(colors);
    if is_reversed { reverse(gradient) } else { gradient }
}

fn analogous(entropy: &mut BitEnumerator, hue_generator: &ColorFunc) -> ColorFunc {
    let spectrum1 = entropy.next_frac();
    let spectrum2 = modulo(spectrum1 + 1.0 / 12.0, 1.0);
    let spectrum3 = modulo(spectrum1 + 2.0 / 12.0, 1.0);
    let spectrum4 = modulo(spectrum1 + 3.0 / 12.0, 1.0);
    let advance = entropy.next_frac() * 0.5 + 0.2;
    let is_reversed = entropy.next();

    let color1 = hue_generator(spectrum1);
    let color2 = hue_generator(spectrum2);
    let color3 = hue_generator(spectrum3);
    let color4 = hue_generator(spectrum4);

    let (darkest_color, dark_color, light_color, lightest_color) =
        if color1.luminance() < color4.luminance() {
            (color1, color2, color3, color4)
        } else {
            (color4, color3, color2, color1)
        };

    let adjusted_darkest = darkest_color.darken(advance);
    let adjusted_dark = dark_color.darken(advance / 2.0);
    let adjusted_light = light_color.lighten(advance / 2.0);
    let adjusted_lightest = lightest_color.lighten(advance);

    let gradient = blend(vec![adjusted_darkest, adjusted_dark, adjusted_light, adjusted_lightest]);
    if is_reversed { reverse(gradient) } else { gradient }
}

fn analogous_fiducial(entropy: &mut BitEnumerator) -> ColorFunc {
    let spectrum1 = entropy.next_frac();
    let spectrum2 = modulo(spectrum1 + 1.0 / 10.0, 1.0);
    let spectrum3 = modulo(spectrum1 + 2.0 / 10.0, 1.0);
    let is_tint = entropy.next();
    let neutral_insert_index = (entropy.next_uint8() % 2 + 1) as usize;
    let is_reversed = entropy.next();

    let neutral_color = if is_tint { Color::WHITE } else { Color::BLACK };

    let spec = spectrum_cmyk_safe();
    let mut colors = vec![spec(spectrum1), spec(spectrum2), spec(spectrum3)];

    match neutral_insert_index {
        1 => {
            colors[0] = adjust_for_luminance(&colors[0], &neutral_color);
            colors[1] = adjust_for_luminance(&colors[1], &neutral_color);
            colors[2] = adjust_for_luminance(&colors[2], &colors[1]);
        }
        2 => {
            colors[1] = adjust_for_luminance(&colors[1], &neutral_color);
            colors[2] = adjust_for_luminance(&colors[2], &neutral_color);
            colors[0] = adjust_for_luminance(&colors[0], &colors[1]);
        }
        _ => panic!("Internal error"),
    }

    colors.insert(neutral_insert_index, neutral_color);

    let gradient = blend(colors);
    if is_reversed { reverse(gradient) } else { gradient }
}

pub fn select_gradient(entropy: &mut BitEnumerator, version: Version) -> ColorFunc {
    if version == Version::GrayscaleFiducial {
        return select_grayscale(entropy);
    }

    let value = entropy.next_uint2();

    match value {
        0 => match version {
            Version::Version1 => {
                let hue_gen: ColorFunc = Box::new(make_hue);
                monochromatic(entropy, &hue_gen)
            }
            Version::Version2 | Version::Detailed => {
                let spec = spectrum_cmyk_safe();
                monochromatic(entropy, &spec)
            }
            Version::Fiducial => monochromatic_fiducial(entropy),
            Version::GrayscaleFiducial => grayscale(),
        },
        1 => match version {
            Version::Version1 => {
                let spec = spectrum();
                complementary(entropy, &spec)
            }
            Version::Version2 | Version::Detailed => {
                let spec = spectrum_cmyk_safe();
                complementary(entropy, &spec)
            }
            Version::Fiducial => complementary_fiducial(entropy),
            Version::GrayscaleFiducial => grayscale(),
        },
        2 => match version {
            Version::Version1 => {
                let spec = spectrum();
                triadic(entropy, &spec)
            }
            Version::Version2 | Version::Detailed => {
                let spec = spectrum_cmyk_safe();
                triadic(entropy, &spec)
            }
            Version::Fiducial => triadic_fiducial(entropy),
            Version::GrayscaleFiducial => grayscale(),
        },
        3 => match version {
            Version::Version1 => {
                let spec = spectrum();
                analogous(entropy, &spec)
            }
            Version::Version2 | Version::Detailed => {
                let spec = spectrum_cmyk_safe();
                analogous(entropy, &spec)
            }
            Version::Fiducial => analogous_fiducial(entropy),
            Version::GrayscaleFiducial => grayscale(),
        },
        _ => grayscale(),
    }
}
