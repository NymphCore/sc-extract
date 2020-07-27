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

use super::{
    errors::{DecompressionError, UnknownPixel},
    utils::{decompress, Reader},
};
use colored::Colorize;
use image::{Rgba, RgbaImage};
use std::path::Path;

/// Reads some data from the stream and returns appropriate pixel data.
///
/// The bitwise transformations depend on the type of the pixel. One of the following
/// types is valid: `0, 1, 2, 3, 4, 6, 10`.
///
/// If `pixel_type` is not one of the above, `UnknownPixel` is raised. Otherwise, an array
/// of four `u8`s is returned, wrapped around by `Ok`.
///
/// ## Arguments
///
/// * `reader`: `Reader` representing the data stream.
/// * `pixel_type`: The type of pixel. For `_tex.sc` data, it is the image sub-type.
fn convert_pixel(reader: &mut Reader, pixel_type: u8) -> Result<[u8; 4], UnknownPixel> {
    match pixel_type {
        // RGB8888
        0 | 1 => {
            let pixel = reader.read(4);
            Ok([pixel[0], pixel[1], pixel[2], pixel[3]])
        }
        // RGB4444
        2 => {
            let pixel = reader.read_uint16();
            Ok([
                (((pixel >> 12) & 0xF) << 4) as u8,
                (((pixel >> 8) & 0xF) << 4) as u8,
                (((pixel >> 4) & 0xF) << 4) as u8,
                ((pixel & 0xF) << 4) as u8,
            ])
        }
        // RGBA5551
        3 => {
            let pixel = reader.read_uint16();
            Ok([
                (((pixel >> 11) & 0x1F) << 3) as u8,
                (((pixel >> 6) & 0x1F) << 3) as u8,
                (((pixel >> 1) & 0x1F) << 3) as u8,
                ((pixel & 0xFF) << 7) as u8,
            ])
        }
        // RGB565
        4 => {
            let pixel = reader.read_uint16();
            Ok([
                (((pixel >> 11) & 0x1F) << 3) as u8,
                (((pixel >> 5) & 0x3F) << 2) as u8,
                ((pixel & 0x1F) << 3) as u8,
                // Alpha channel must always be 255 for type 4.
                255
            ])
        }
        // LA88
        6 => {
            let pixel = reader.read_uint16();
            Ok([
                (pixel >> 8) as u8,
                (pixel >> 8) as u8,
                (pixel >> 8) as u8,
                (pixel & 0xFF) as u8,
            ])
        }
        10 => {
            let pixel = reader.read_byte();
            Ok([pixel; 4])
        }
        _ => Err(UnknownPixel(format!(
            "Unknown pixel type ({}).",
            pixel_type
        ))),
    }
}

/// Adjusts some pixels.
fn adjust_pixels(img: &mut RgbaImage, pixels: Vec<[u8; 4]>, height: u16, width: u16) {
    let mut i = 0;
    let block_size = 32;
    let h_limit = (height as f64 / block_size as f64).ceil() as u32;
    let w_limit = (width as f64 / block_size as f64).ceil() as u32;

    for _h in 0..h_limit {
        for _w in 0..w_limit {
            let mut h = _h * block_size;
            while h != (_h + 1) * block_size && h < height as u32 {
                let mut w = _w * block_size;
                while w != (_w + 1) * block_size && w < width as u32 {
                    img.put_pixel(
                        w,
                        h,
                        Rgba([pixels[i][0], pixels[i][1], pixels[i][2], pixels[i][3]]),
                    );
                    i += 1;
                    w += 1;
                }
                h += 1;
            }
        }
    }
}

/// Processes compressed, raw `_tex.sc` file data.
///
/// If decompressing and pixel conversion is successful, the resultant image
/// is saved in `PNG` format in the output directory (`out_dir`).
///
/// If decompression is unsuccessful, `DecompressionError` is raised. Pixel
/// conversion errors are handled in the function itself.
///
/// A single `_tex.sc` file can contain data for multiple sprites. All of the sprites
/// are extracted and saved by this process. `_`s are appended to the file name in cases
/// of multiple sprites.
///
/// ## Arguments
///
/// * `data`: Raw `_tex.sc` file data.
/// * `path`: Path to the `_tex.sc` file. It is used to get file name.
/// * `out_dir`: Directory to store extracted images.
/// * `parallelize`: Whether files are processed in parallel or not.
pub fn process_sc(data: &[u8], path: &Path, out_dir: &Path, parallelize: bool) -> Result<(), DecompressionError> {
    if data.len() < 35 {
        return Err(DecompressionError("Size of file is too small:".to_string()));
    }

    let decompressed = match decompress(&data[26..]) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    let mut reader = Reader::new(decompressed);

    let mut pic_count = 0;
    let possible_types = [1, 24, 27, 28];

    let file_name = path.file_stem().unwrap().to_str().unwrap();

    if !parallelize {
        println!(
            "\nExtracting {} image(s)...",
            path.file_name().unwrap().to_str().unwrap().green().bold()
        );
    }

    'main: while reader.len() > 0 {
        let file_type = reader.read_byte();
        let file_size = reader.read_uint32();

        if !possible_types.contains(&file_type) {
            reader.read(file_size as usize);
            continue;
        }

        let sub_type = reader.read_byte();
        let width = reader.read_uint16();
        let height = reader.read_uint16();

        println!(
            "file_type: {}, file_size: {}, sub_type: {}, width: {}, height: {}",
            file_type.to_string().cyan().bold(),
            file_size.to_string().cyan().bold(),
            sub_type.to_string().cyan().bold(),
            width.to_string().cyan().bold(),
            height.to_string().cyan().bold()
        );

        let mut pixels = Vec::new();
        let mut img = RgbaImage::new(width as u32, height as u32);
        for y in 0..height {
            for x in 0..width {
                let [one, two, three, four] = match convert_pixel(&mut reader, sub_type) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("Error: {}", e.0.red());
                        continue 'main;
                    }
                };
                pixels.push([one, two, three, four]);
                // img.put_pix
                img.put_pixel(x as u32, y as u32, Rgba([one, two, three, four]));
            }
        }

        if file_type == 27 || file_type == 28 {
            adjust_pixels(&mut img, pixels, height, width);
        }

        let initial_path = out_dir.join(file_name);
        let path = format!(
            "{}{}.png",
            initial_path.to_str().unwrap(),
            "_".repeat(pic_count)
        );
        img.save(path)
            .unwrap_or_else(|_| panic!("Failed to save image!".red()));

        pic_count += 1;
    }

    Ok(())
}
