use std::{env, process::ExitCode};

use nifti::{NiftiImage, RawNiftiImage};

fn main() -> ExitCode {
    let Some(path) = env::args().nth(1) else {
        eprintln!("usage: cargo run -p nifti-playground -- <image.nii | image.nii.gz>");
        return ExitCode::FAILURE;
    };

    if let Err(error) = run(&path) {
        eprintln!("error: {error:?}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn run(path: &str) -> nifti::Result<()> {
    // Current compatibility API: NiftiImage is an alias for RawNiftiImage.
    let image = NiftiImage::read_with_data(path)?;

    println!("loaded with data");
    println!("dimensions: {:?}", image.dimensions());
    println!("datatype: {}", image.datatype());
    println!("voxel count: {}", image.voxel_count());
    println!("has data: {}", image.has_data());

    // Header-only path, useful for future lazy/chunked APIs.
    let mut header_only = RawNiftiImage::read_header(path)?;

    println!();
    println!("loaded header only");
    println!("dimensions: {:?}", header_only.dimensions());
    println!("datatype: {}", header_only.datatype());
    println!("voxel count: {}", header_only.voxel_count());
    println!("has data before load: {}", header_only.has_data());

    header_only.load_data()?;
    println!("has data after load: {}", header_only.has_data());

    header_only.unload_data();
    println!("has data after unload: {}", header_only.has_data());

    Ok(())
}
