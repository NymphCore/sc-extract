// MIT License

// Copyright (c) 2020 AriusX7

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use super::{errors::DecompressionError, utils::decompress};
use colored::Colorize;
use std::{fs, path::Path};

/// Process raw, encoded `.csv` file data.
///
/// The data passed here must be **compressed/raw**. Passing uncompressed or decoded
/// csv file data will result in `DecompressionError`.
///
/// If decompression is unsuccessful, `DecompressionError` is raised.
///
/// ## Arguments
///
/// * `data`: Raw `.csv` file data.
/// * `path`: Path to the `.csv` file. It is used to get file name.
/// * `out_dir`: Directory to store extracted csv files.
pub fn process_csv(data: &[u8], path: &Path, out_dir: &Path) -> Result<(), DecompressionError> {
    let decompressed = match decompress(data) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    let file_name = path.file_name().unwrap().to_str().unwrap();

    println!("\nExtracting {} file...", file_name.green().bold());

    fs::write(out_dir.join(file_name), decompressed.get_ref()).unwrap();

    Ok(())
}
