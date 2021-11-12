use thiserror::Error;

/// Errors returned while loading/parsing a serialized SymCache.
///
/// After a SymCache was successfully parsed via [`Format::parse`], an Error that occurs during
/// access of any data indicates either corruption of the serialized file, or a bug in the
/// converter/serializer.
#[derive(Debug, Error)]
pub enum Error {
    /// The buffer is not correctly aligned.
    #[error("source buffer is not correctly aligned")]
    BufferNotAligned,
    /// The header's size doesn't match our expected size.
    #[error("header is too small")]
    HeaderTooSmall,
    /// The file was generated by a system with different endianness.
    #[error("endianness mismatch")]
    WrongEndianness,
    /// The file magic does not match.
    #[error("wrong format magic")]
    WrongFormat,
    /// The format version in the header is wrong/unknown.
    #[error("unknown symcache version")]
    WrongVersion,
    /// The self-advertised size of the buffer is not correct.
    #[error("incorrect buffer length")]
    BadFormatLength,
    /// The file index is out of bounds.
    #[error("file index {0} out of bounds")]
    InvalidFileReference(u32),
    /// The function index is out of bounds.
    #[error("function index {0} out of bounds")]
    InvalidFunctionReference(u32),
    /// The source location index is out of bounds.
    #[error("source location index {0} out of bounds")]
    InvalidSourceLocationReference(u32),
    /// The string index is out of bounds.
    #[error("string index {0} out of bounds")]
    InvalidStringReference(u32),
    /// The string data is out of bounds.
    #[error("string data {0} out of bounds")]
    InvalidStringDataReference(u32),
    /// The string data is invalid UTF-8.
    #[error("string data {0} contains invalid UTF-8")]
    InvalidStringData(u32, std::str::Utf8Error),
}
