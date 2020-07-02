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

use colored::Colorize;
use rayon::prelude::*;
use sc_extract::{csv::process_csv, tex::process_sc};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    /// The path to directory containing `_tex.sc` files or
    /// path to an `_tex.sc` file.
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    /// The path to directory where an extracts folder is created to save output.
    /// If not specified, `extracts` is created inside `path`.
    #[structopt(parse(from_os_str), short = "o", long = "out")]
    out_dir: Option<PathBuf>,

    /// If this flag is supplied, the source `_tex.sc` files are deleted after extracting.
    #[structopt(short = "d", long = "delete")]
    delete: bool,
}

/// Checks if file path ends with `_tex.sc` or `.csv`.
fn is_valid_file(path: &PathBuf) -> bool {
    path.to_str().unwrap().ends_with("_tex.sc") || path.to_str().unwrap().ends_with(".csv")
}

/// Deletes the file with given path.
/// It deletion fails, prints it on stdout.
fn delete_file(path: &PathBuf) {
    match fs::remove_file(&path) {
        Ok(_) => (),
        Err(_) => println!(
            "{}: {}",
            "Failed to remove file".red(),
            path.to_str().unwrap().red()
        ),
    };
}

/// Checks if data has correct header and returns Option<header_type>,
/// where header_type can be "sc" or "csv", depending on the data.
///
/// The data passed here must be compressed/raw.
fn check_header(data: &[u8]) -> Option<&'static str> {
    if data.is_empty() {
        None
    } else if data[0] == 83 {
        Some("sc")
    } else if data[..2] == [93, 0] {
        Some("csv")
    } else {
        None
    }
}

/// Processes the given file (path).
///
/// It automatically detects file type (`_tex.sc` or `.csv`) and processes them appropriately.
/// If processing a file fails, formatted error messages gets printed on `stdout`.
/// In case of lack of permissions, the process may panic.
///
/// ## Arguments
///
/// * `path`: Reference to the file path.
/// * `out_dir`: Path to directory where `extracts` folder is created to
///     store extracts.
/// * `delete`: Whether to delete file after extraction or not.
fn process_file(path: &PathBuf, out_dir: &PathBuf, delete: bool) -> Result<(), ()> {
    let data = fs::read(&path).unwrap();

    let process = check_header(data.as_slice());

    let process_fn = match process {
        Some("sc") => process_sc,
        Some("csv") => process_csv,
        _ => {
            println!(
                "{}",
                format!(
                    "File has `_tex.sc` or `.csv` extension but is actually of unknown type: {}",
                    path.to_str().unwrap()
                )
                .red()
            );
            return Err(());
        }
    };

    match process_fn(&data, &path, &out_dir) {
        Ok(_) => (),
        Err(e) => println!("\n{} {}", e.0.red(), path.to_str().unwrap().red()),
    };

    if delete {
        delete_file(&path);
    }

    Ok(())
}

fn main() {
    let opts = Options::from_args();

    let out_dir = match &opts.out_dir {
        Some(p) => p.join("extracts"),
        None => {
            if opts.path.is_dir() {
                opts.path.join("extracts")
            } else if opts.path.is_file() {
                opts.path.parent().unwrap().join("extracts")
            } else {
                std::env::current_dir().unwrap().join("extracts")
            }
        }
    };

    if !out_dir.exists() {
        fs::create_dir_all(&out_dir).unwrap();
    }

    if opts.path.is_dir() {
        let found_one = AtomicBool::new(false);
        let dir_entries = match fs::read_dir(&opts.path) {
            Ok(e) => e,
            Err(_) => {
                println!(
                    "{}",
                    format!(
                        "Failed to read contents of {} directory/folder.",
                        opts.path.to_str().unwrap().red()
                    )
                    .red()
                );
                std::process::exit(1);
            }
        };
        let mut entries = Vec::new();
        for entry in dir_entries {
            entries.push(entry);
        }
        entries.into_par_iter().for_each(|entry| {
            let path = entry.unwrap().path();
            if is_valid_file(&path) && process_file(&path, &out_dir, opts.delete).is_ok() {
                found_one.compare_and_swap(false, true, Ordering::AcqRel);
            }
        });
        if !found_one.into_inner() {
            println!(
                "{}",
                "No `_tex.sc` or `.csv` file in the given directory!".red()
            );
            std::process::exit(1);
        }
    } else if opts.path.is_file() {
        if is_valid_file(&opts.path) {
            process_file(&opts.path, &out_dir, opts.delete).unwrap_or(());
        } else {
            println!(
                "{}",
                "Given file doesn't appear to be an `_tex.sc` or `.csv` file!".red()
            );
            std::process::exit(1);
        }
    }

    println!("\n{}", "Extraction finished!".green().bold());
}
