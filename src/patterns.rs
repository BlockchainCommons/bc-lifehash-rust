use crate::{Version, bit_enumerator::BitEnumerator};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Pattern {
    Snowflake,
    Pinwheel,
    Fiducial,
}

pub fn select_pattern(
    entropy: &mut BitEnumerator,
    version: Version,
) -> Pattern {
    match version {
        Version::Fiducial | Version::GrayscaleFiducial => Pattern::Fiducial,
        _ => {
            if entropy.next() {
                Pattern::Snowflake
            } else {
                Pattern::Pinwheel
            }
        }
    }
}
