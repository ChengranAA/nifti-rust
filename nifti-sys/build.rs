use std::{env, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let nifti_clib = manifest_dir.join("nifti_clib");

    if !nifti_clib.join("CMakeLists.txt").exists() {
        panic!("nifti_clib submodule is missing. Run `git submodule update --init --recursive`.");
    }

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=nifti_clib/nifti2/nifti1.h");
    println!("cargo:rerun-if-changed=nifti_clib/nifti2/nifti2.h");
    println!("cargo:rerun-if-changed=nifti_clib/nifti2/nifti2_io.h");
    println!("cargo:rerun-if-changed=nifti_clib/nifti2/nifti2_io.c");
    println!("cargo:rerun-if-changed=nifti_clib/znzlib/znzlib.h");
    println!("cargo:rerun-if-changed=nifti_clib/znzlib/znzlib.c");

    let dst = cmake::Config::new(&nifti_clib)
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("NIFTI_BUILD_APPLICATIONS", "OFF")
        .define("BUILD_TESTING", "OFF")
        .define("NIFTI_BUILD_TESTING", "OFF")
        .define("USE_NIFTICDF_CODE", "OFF")
        .define("USE_NIFTI2_CODE", "ON")
        .define("USE_FSL_CODE", "OFF")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=nifti2");
    println!("cargo:rustc-link-lib=static=znz");

    // nifti_clib's znz library is built with zlib support by default.
    println!("cargo:rustc-link-lib=z");

    if !env::var("CARGO_CFG_WINDOWS").is_ok() {
        println!("cargo:rustc-link-lib=m");
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", nifti_clib.join("nifti2").display()))
        .clang_arg(format!("-I{}", nifti_clib.join("znzlib").display()))
        .clang_arg("-DHAVE_ZLIB")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to generate NIfTI bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write NIfTI bindings");
}
