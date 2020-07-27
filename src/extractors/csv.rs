use crate::{errors::DecompressionError, utils::decompress};
use colored::Colorize;
use std::{fs, path::Path};

/// Processes encoded, raw `.csv` file data.
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
/// * `_parallelize`: Whether files are processed in parallel or not.
///   Has no effect on this function.
pub fn process_csv(
    data: &[u8],
    path: &Path,
    out_dir: &Path,
    _parallelize: bool,
) -> Result<(), DecompressionError> {
    let decompressed = match decompress(data) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    let file_name = path.file_name().unwrap().to_str().unwrap();

    println!("\nExtracting {} file...", file_name.green().bold());

    fs::write(out_dir.join(file_name), decompressed.get_ref()).unwrap();

    Ok(())
}
