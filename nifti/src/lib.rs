use std::{ffi::CString, ptr::NonNull};

use nifti_sys::{nifti_image, nifti_image_free, nifti_image_read};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    PathContainsNul,
    ReadFailed,
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct NiftiImage {
    raw: NonNull<nifti_image>,
}

impl NiftiImage {
    pub fn read(path: &str) -> Result<Self> {
        let path = CString::new(path).map_err(|_| Error::PathContainsNul)?;

        let raw = unsafe { nifti_image_read(path.as_ptr(), 1) };
        let raw = NonNull::new(raw).ok_or(Error::ReadFailed)?;

        Ok(Self { raw })
    }

    pub fn dimensions(&self) -> [i64; 8] {
        let image = unsafe { self.raw.as_ref() };

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

    pub fn datatype(&self) -> i32 {
        unsafe { self.raw.as_ref().datatype }
    }

    pub fn voxel_count(&self) -> i64 {
        unsafe { self.raw.as_ref().nvox }
    }

    pub fn as_raw(&self) -> *const nifti_image {
        self.raw.as_ptr()
    }

    pub fn as_raw_mut(&mut self) -> *mut nifti_image {
        self.raw.as_ptr()
    }
}

impl Drop for NiftiImage {
    fn drop(&mut self) {
        unsafe {
            nifti_image_free(self.raw.as_ptr());
        }
    }
}
