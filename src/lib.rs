//! Library to extract compressed/encoded raw `_tex.sc` and `.csv` files used in Supercell games.
//!
//! The library exposes two high-level functions, `process_sc` and `process_csv`, to process
//! `_tex.sc` and `.csv` files respectively. The functions only process valid, compressed/encoded
//! and raw files found directly in Supercell apps.
//!
//! This library is simply intended to get high quality graphics and data from the files.
//! It is in no way an attempt to:
//!
//! - modify the game in any way
//! - create a clone or any other game based on Supercell games
//! - make profit

pub mod errors;
mod extractors;
mod utils;

#[doc(inline)]
pub use extractors::{csv::process_csv, tex::process_sc};
