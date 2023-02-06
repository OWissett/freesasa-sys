extern crate autotools;
extern crate bindgen;
use fs_extra::dir;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Ensure that the freesasa directory exists
    if !PathBuf::from("freesasa").exists() {
        println!("cargo:warning=The freesasa directory did not exist, automatically initializing submodules");
        std::process::Command::new("git")
            .args(&["submodule", "update", "--init", "--recursive"])
            .output()
            .expect("Failed to execute git command");
    }

    // copy freesasa to OUT_DIR - this is needed because the build script
    // modifies the contents of the freesasa directory, which is not allowed
    // by docs.rs, we can only modify the OUT_DIR
    let out_dir = env::var("OUT_DIR").unwrap();
    let freesasa_dir = PathBuf::from(&out_dir).join("freesasa");
    let freesasa_src = PathBuf::from("freesasa");

    let mut copy_options = dir::CopyOptions::new();
    copy_options.overwrite = true;

    dir::copy(&freesasa_src, &out_dir, &copy_options).unwrap();

    // Use autotools to compile the native library
    let dst = autotools::Config::new(&freesasa_dir)
        .reconf("-i")
        .config_option("disable-json", None)
        .config_option("disable-xml", None)
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=freesasa");

    let header = env::var("OUT_DIR").unwrap() + "/include/freesasa.h";

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .header(header)
        .merge_extern_blocks(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Delete freesasa directory
    dir::remove(&freesasa_dir).unwrap();
}
