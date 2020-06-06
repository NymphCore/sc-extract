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

use super::errors::DecompressionError;
use byteorder::{LittleEndian, ReadBytesExt};
use lzma_rs::lzma_decompress;
use std::io::{Cursor, Read};

/// Wrapper for reading data from stream.
pub(crate) struct Reader {
    stream: Cursor<Vec<u8>>,
    bytes_left: usize,
}

impl Reader {
    /// Create new `Reader` instance from a stream.
    pub fn new(stream: Cursor<Vec<u8>>) -> Self {
        let bytes_left = stream.get_ref().len();

        Self { stream, bytes_left }
    }

    /// Bytes left in the data stream.
    pub fn len(&self) -> usize {
        self.bytes_left
    }

    /// Read exact number of bytes from the stream.
    pub fn read(&mut self, size: usize) -> Vec<u8> {
        if size > self.bytes_left {
            self.bytes_left = 0;
        } else {
            self.bytes_left -= size;
        }

        let mut buf = vec![0; size];
        if self.bytes_left == 0 {
            self.stream.read_to_end(&mut buf).unwrap();

            buf
        } else {
            self.stream.read_exact(&mut buf).unwrap();

            buf
        }
    }

    /// Read one byte from the stream.
    pub fn read_byte(&mut self) -> u8 {
        if 1 > self.bytes_left {
            self.bytes_left = 0;
        } else {
            self.bytes_left -= 1;
        }

        self.stream.read_u8().unwrap()
    }

    /// Read an unsigned 16-bit little-endian integer from the stream.
    pub fn read_uint16(&mut self) -> u16 {
        if 2 > self.bytes_left {
            self.bytes_left = 0;
        } else {
            self.bytes_left -= 2;
        }

        self.stream.read_u16::<LittleEndian>().unwrap()
    }

    /// Read an unsigned 32-bit little-endian integer from the stream.
    pub fn read_uint32(&mut self) -> u32 {
        if 4 > self.bytes_left {
            self.bytes_left = 0;
        } else {
            self.bytes_left -= 4;
        }

        self.stream.read_u32::<LittleEndian>().unwrap()
    }

    /// Read an signed 16-bit little-endian integer from the stream.
    pub fn _read_int16(&mut self) -> i16 {
        if 2 > self.bytes_left {
            self.bytes_left = 0;
        } else {
            self.bytes_left -= 2;
        }

        self.stream.read_i16::<LittleEndian>().unwrap()
    }

    /// Read an signed 32-bit little-endian integer from the stream.
    pub fn _read_int32(&mut self) -> i32 {
        if 4 > self.bytes_left {
            self.bytes_left = 0;
        } else {
            self.bytes_left -= 4;
        }

        self.stream.read_i32::<LittleEndian>().unwrap()
    }

    /// Read `length` bytes from the stream and return the output as a `String`.
    pub fn _read_string(&mut self, length: usize) -> String {
        if length > self.bytes_left {
            self.bytes_left = 0;
        } else {
            self.bytes_left -= length;
        }

        String::from_utf8_lossy(self.read(length).as_slice()).to_string()
    }

    /// Returns the current position of this cursor as usize.
    pub fn _tell(&self) -> usize {
        self.stream.position() as usize
    }
}

/// Decompress given raw `.tex_sc` or `.csv` data.
///
/// Before decompressing the data using `LZMA` decompression,
/// four `\x00` bytes are added to `raw_data` after the eigth index.
/// A `Curson` containing the transformed raw data is returned, wrapped up in `Ok`.
///
/// If the decompression fails due to any reason, `DecompressionError` is raised.
/// 
/// ## Arguments
/// * `raw_data`: The raw `_tex.sc` or `.csv` file data.
pub(crate) fn decompress(raw_data: &[u8]) -> Result<Cursor<Vec<u8>>, DecompressionError> {
    let mut data = raw_data[0..9].to_vec();

    data.append(&mut vec![b'\x00'; 4]);
    data.append(raw_data[9..].to_vec().as_mut());

    let mut decomp: Vec<u8> = Vec::new();
    match lzma_decompress(&mut data.as_slice(), &mut decomp) {
        Ok(_) => Ok(Cursor::new(decomp)),
        Err(_) => Err(DecompressionError("Failed to decompress file:".to_owned())),
    }
}
