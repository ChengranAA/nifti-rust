use std::{ffi::CString, ptr::NonNull};

use nifti_sys::{
    nifti_image, nifti_image_free, nifti_image_infodump, nifti_image_load, nifti_image_read,
    nifti_image_unload, nifti_image_write_status,
};

use crate::{Error, Result};

/// Owned wrapper around a `nifti_image *` allocated by the NIfTI C library.
///
/// This type is intentionally thin: it owns the raw image pointer, calls the
/// matching C destructor on drop, and exposes small safe wrappers for common
/// `nifti_image_*` operations. Higher-level Rust types should convert data out
/// of this type instead of exposing `nifti_sys` directly.
pub struct RawNiftiImage {
    raw: NonNull<nifti_image>,
}

impl RawNiftiImage {
    /// Reads a NIfTI image from disk.
    ///
    /// If `read_data` is `false`, only the header/metadata is read. The image
    /// data can be loaded later with [`Self::load_data`].
    pub fn read(path: &str, read_data: bool) -> Result<Self> {
        let path = CString::new(path).map_err(|_| Error::PathContainsNul)?;
        let read_data = i32::from(read_data);

        let raw = unsafe { nifti_image_read(path.as_ptr(), read_data) };
        unsafe { Self::from_raw(raw) }.ok_or(Error::ReadFailed)
    }

    /// Reads both header and voxel data from disk.
    pub fn read_with_data(path: &str) -> Result<Self> {
        Self::read(path, true)
    }

    /// Reads only the image header/metadata from disk.
    pub fn read_header(path: &str) -> Result<Self> {
        Self::read(path, false)
    }

    /// Takes ownership of a raw `nifti_image *` returned by the C library.
    ///
    /// Returns `None` if `raw` is null.
    ///
    /// # Safety
    ///
    /// `raw` must either be null or point to a valid `nifti_image` allocation
    /// that must be released with `nifti_image_free`. After passing a non-null
    /// pointer here, no other owner may free it.
    pub unsafe fn from_raw(raw: *mut nifti_image) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    /// Loads voxel data for a header-only image.
    pub fn load_data(&mut self) -> Result<()> {
        let status = unsafe { nifti_image_load(self.raw.as_ptr()) };

        if status == 0 {
            Ok(())
        } else {
            Err(Error::LoadFailed)
        }
    }

    /// Unloads voxel data while keeping the image header alive.
    pub fn unload_data(&mut self) {
        unsafe {
            nifti_image_unload(self.raw.as_ptr());
        }
    }

    /// Writes the image using the filename/state stored in the C image object.
    pub fn write(&mut self) -> Result<()> {
        let status = unsafe { nifti_image_write_status(self.raw.as_ptr()) };

        if status == 0 {
            Ok(())
        } else {
            Err(Error::WriteFailed)
        }
    }

    /// Dumps image metadata using the C library's diagnostic printer.
    pub fn infodump(&self) {
        unsafe {
            nifti_image_infodump(self.raw.as_ptr());
        }
    }

    /// Returns the logical dimensions `[nx, ny, nz, nt, nu, nv, nw, ndim]`.
    pub fn dimensions(&self) -> [i64; 8] {
        let image = self.as_ref();

        [
            image.nx,
            image.ny,
            image.nz,
            image.nt,
            image.nu,
            image.nv,
            image.nw,
            image.dim[0],
        ]
    }

    /// Returns the raw NIfTI datatype code.
    pub fn datatype(&self) -> i32 {
        self.as_ref().datatype
    }

    /// Returns the number of voxels reported by the C image object.
    pub fn voxel_count(&self) -> i64 {
        self.as_ref().nvox
    }

    /// Returns whether the C image object currently has a non-null data pointer.
    pub fn has_data(&self) -> bool {
        !self.as_ref().data.is_null()
    }

    /// Returns the raw image pointer for read-only FFI interop.
    pub fn as_ptr(&self) -> *const nifti_image {
        self.raw.as_ptr()
    }

    /// Returns the raw image pointer for mutable FFI interop.
    pub fn as_mut_ptr(&mut self) -> *mut nifti_image {
        self.raw.as_ptr()
    }

    /// Compatibility helper for the initial API.
    pub fn as_raw(&self) -> *const nifti_image {
        self.as_ptr()
    }

    /// Compatibility helper for the initial API.
    pub fn as_raw_mut(&mut self) -> *mut nifti_image {
        self.as_mut_ptr()
    }

    /// Borrows the underlying C image struct.
    pub fn as_ref(&self) -> &nifti_image {
        unsafe { self.raw.as_ref() }
    }

    /// Mutably borrows the underlying C image struct.
    pub fn as_mut(&mut self) -> &mut nifti_image {
        unsafe { self.raw.as_mut() }
    }
}

impl Drop for RawNiftiImage {
    fn drop(&mut self) {
        unsafe {
            nifti_image_free(self.raw.as_ptr());
        }
    }
}
