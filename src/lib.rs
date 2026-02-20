#![doc(html_root_url = "https://docs.rs/bc-lifehash/0.1.0")]
#![warn(rust_2018_idioms)]

//! # Introduction
//!
//! `bc-lifehash` is a method of hash visualization based on Conway's Game of
//! Life that creates beautiful icons that are deterministic, yet distinct and
//! unique given the input data.
//!
//! The basic concept is to take a SHA-256 hash of the input data (which can be
//! any data including another hash) and then use the 256-bit digest as a 16x16
//! pixel "seed" for running the cellular automata known as Conway's Game of
//! Life. After the pattern becomes stable (or begins repeating) the resulting
//! history is used to compile a grayscale image of all the states from the
//! first to last generation. Using Game of Life provides visual structure to
//! the resulting image, even though it was seeded with entropy. Some bits of
//! the initial hash are then used to deterministically apply symmetry and color
//! to the icon to add beauty and quick recognizability.
//!
//! This is the first-party Rust implementation. It produces byte-identical
//! output to the [C++ reference implementation](https://github.com/BlockchainCommons/bc-lifehash).
//!
//! # Getting Started
//!
//! ```toml
//! [dependencies]
//! bc-lifehash = "0.1.0"
//! ```
//!
//! # Versions
//!
//! Five LifeHash versions are supported via the [`Version`] enum:
//!
//! - **Version1** / **Version2** — 16x16 grid, up to 150 generations.
//! - **Detailed** — 32x32 grid, up to 300 generations, richer color gradients.
//! - **Fiducial** — 32x32, designed for use as fiducial markers.
//! - **GrayscaleFiducial** — Same as Fiducial but rendered in grayscale.
//!
//! # Example
//!
//! ```rust
//! let image = bc_lifehash::make_from_utf8("Hello", bc_lifehash::Version::Version2, 1, false);
//! assert_eq!(image.width, 32);
//! assert_eq!(image.height, 32);
//! // image.colors contains RGB bytes (width * height * 3)
//! ```

mod bit_enumerator;
mod cell_grid;
mod change_grid;
mod color;
mod color_func;
mod color_grid;
mod frac_grid;
mod gradients;
mod grid;
mod hsb_color;
mod patterns;

use std::collections::BTreeSet;

use sha2::{Digest, Sha256};

use bit_enumerator::BitEnumerator;
use cell_grid::CellGrid;
use change_grid::ChangeGrid;
use color::clamped;
use color::lerp_from;
use color_grid::ColorGrid;
use frac_grid::FracGrid;
use gradients::select_gradient;
use patterns::select_pattern;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Version {
    Version1,
    Version2,
    Detailed,
    Fiducial,
    GrayscaleFiducial,
}

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub colors: Vec<u8>,
}

fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn make_image(
    width: usize,
    height: usize,
    float_colors: &[f64],
    module_size: usize,
    has_alpha: bool,
) -> Image {
    assert!(module_size > 0, "Invalid module size");

    let scaled_width = width * module_size;
    let scaled_height = height * module_size;
    let result_components = if has_alpha { 4 } else { 3 };
    let scaled_capacity = scaled_width * scaled_height * result_components;

    let mut result_colors = vec![0u8; scaled_capacity];

    // Match C++ loop order: outer loop uses scaled_width, inner uses scaled_height
    // (they're swapped relative to the variable names, but since the image is always
    // square this doesn't matter in practice)
    for target_y in 0..scaled_width {
        for target_x in 0..scaled_height {
            let source_x = target_x / module_size;
            let source_y = target_y / module_size;
            let source_offset = (source_y * width + source_x) * 3;

            let target_offset = (target_y * scaled_width + target_x) * result_components;

            result_colors[target_offset] = (clamped(float_colors[source_offset]) * 255.0) as u8;
            result_colors[target_offset + 1] =
                (clamped(float_colors[source_offset + 1]) * 255.0) as u8;
            result_colors[target_offset + 2] =
                (clamped(float_colors[source_offset + 2]) * 255.0) as u8;
            if has_alpha {
                result_colors[target_offset + 3] = 255;
            }
        }
    }

    Image {
        width: scaled_width,
        height: scaled_height,
        colors: result_colors,
    }
}

pub fn make_from_utf8(
    s: &str,
    version: Version,
    module_size: usize,
    has_alpha: bool,
) -> Image {
    make_from_data(s.as_bytes(), version, module_size, has_alpha)
}

pub fn make_from_data(
    data: &[u8],
    version: Version,
    module_size: usize,
    has_alpha: bool,
) -> Image {
    let digest = sha256(data);
    make_from_digest(&digest, version, module_size, has_alpha)
}

pub fn make_from_digest(
    digest: &[u8],
    version: Version,
    module_size: usize,
    has_alpha: bool,
) -> Image {
    assert_eq!(digest.len(), 32, "Digest must be 32 bytes");

    let (length, max_generations): (usize, usize) = match version {
        Version::Version1 | Version::Version2 => (16, 150),
        Version::Detailed | Version::Fiducial | Version::GrayscaleFiducial => (32, 300),
    };

    let mut current_cell_grid = CellGrid::new(length, length);
    let mut next_cell_grid = CellGrid::new(length, length);
    let mut current_change_grid = ChangeGrid::new(length, length);
    let mut next_change_grid = ChangeGrid::new(length, length);

    match version {
        Version::Version1 => {
            next_cell_grid.set_data(digest);
        }
        Version::Version2 => {
            let hashed = sha256(digest);
            next_cell_grid.set_data(&hashed);
        }
        Version::Detailed | Version::Fiducial | Version::GrayscaleFiducial => {
            let mut digest1 = digest.to_vec();
            if version == Version::GrayscaleFiducial {
                digest1 = sha256(&digest1);
            }
            let digest2 = sha256(&digest1);
            let digest3 = sha256(&digest2);
            let digest4 = sha256(&digest3);
            let mut digest_final = digest1;
            digest_final.extend_from_slice(&digest2);
            digest_final.extend_from_slice(&digest3);
            digest_final.extend_from_slice(&digest4);
            next_cell_grid.set_data(&digest_final);
        }
    }

    next_change_grid.grid.set_all(true);

    let mut history_set: BTreeSet<Vec<u8>> = BTreeSet::new();
    let mut history: Vec<Vec<u8>> = Vec::new();

    while history.len() < max_generations {
        std::mem::swap(&mut current_cell_grid, &mut next_cell_grid);
        std::mem::swap(&mut current_change_grid, &mut next_change_grid);

        let data = current_cell_grid.data();
        let hash = sha256(&data);
        if history_set.contains(&hash) {
            break;
        }
        history_set.insert(hash);
        history.push(data);

        current_cell_grid.next_generation(
            &current_change_grid,
            &mut next_cell_grid,
            &mut next_change_grid,
        );
    }

    let mut frac_grid = FracGrid::new(length, length);
    for (i, h) in history.iter().enumerate() {
        current_cell_grid.set_data(h);
        let frac = clamped(lerp_from(0.0, history.len() as f64, (i + 1) as f64));
        frac_grid.overlay(&current_cell_grid, frac);
    }

    // Normalize the frac_grid to [0, 1] (except version1)
    if version != Version::Version1 {
        let mut min_value = f64::INFINITY;
        let mut max_value = f64::NEG_INFINITY;
        frac_grid.grid.for_all(|x, y| {
            let value = frac_grid.grid.get_value(x, y);
            if value < min_value {
                min_value = value;
            }
            if value > max_value {
                max_value = value;
            }
        });

        let width = frac_grid.grid.width;
        let height = frac_grid.grid.height;
        for y in 0..height {
            for x in 0..width {
                let value = frac_grid.grid.get_value(x, y);
                let normalized = lerp_from(min_value, max_value, value);
                frac_grid.grid.set_value(normalized, x, y);
            }
        }
    }

    let mut entropy = BitEnumerator::new(digest.to_vec());

    match version {
        Version::Detailed => {
            entropy.next();
        }
        Version::Version2 => {
            entropy.next_uint2();
        }
        _ => {}
    }

    let gradient = select_gradient(&mut entropy, version);
    let pattern = select_pattern(&mut entropy, version);
    let color_grid = ColorGrid::new(&frac_grid, &gradient, pattern);

    make_image(
        color_grid.grid.width,
        color_grid.grid.height,
        &color_grid.colors(),
        module_size,
        has_alpha,
    )
}
