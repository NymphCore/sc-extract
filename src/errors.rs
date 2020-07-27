//! Errors raised during processing of files.

/// Error when an unknown type of pixel is passed.
pub struct UnknownPixel(pub String);

/// Error when `_tex.sc` or `.csv` file decompression fails.
pub struct DecompressionError(pub String);
