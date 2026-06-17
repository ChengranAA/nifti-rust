pub mod raw;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    PathContainsNul,
    ReadFailed,
    LoadFailed,
    WriteFailed,
}

pub type Result<T> = std::result::Result<T, Error>;

pub use raw::RawNiftiImage;

/// Temporary compatibility alias for the initial high-level API.
///
/// As the safe API grows, `NiftiImage` should become a fully-owned Rust image
/// type while `RawNiftiImage` remains the FFI-backed implementation detail.
pub type NiftiImage = RawNiftiImage;
