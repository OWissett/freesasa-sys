//! This crate provides raw FFI bindings to the [freesasa](https://freesasa.github.io/doxygen/index.html)
//! C library, developed by [Simon Mittinatten](https://github.com/mittinatten) \[1\]. FreeSASA allows
//! you to calculate the solvent accessible surface area (SASA) of a protein from its atomic
//! coordinates. The library is written in C, and is available under the MIT license.
//!
//! The bindings in this crate are automatically generated by `bindgen`. Since this crate provides
//! raw access to a C-library, it is on the user to ensure that memory safety is maintained.
//!
//! For most uses it is recommended to use the RustSASA crate (which is currently not publicly available).
//! Please contact me via email, ow257@cam.ac.uk, if you are interested in access to this library.
//!
//!
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(clippy::upper_case_acronyms)]

// This is needed for ensuring that
// we can link to libc++.
extern crate link_cplusplus;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use crate::{
        fopen, freesasa_calc_structure, freesasa_classifier, freesasa_protor_classifier,
        freesasa_structure_from_pdb,
    };
    use std::{ffi, ptr};

    #[test]
    fn freesasa_calculation() {
        unsafe {
            // Define the file name
            let pdb_filename = ffi::CString::new("./test/single_chain.pdb").unwrap();

            // Define the file mode
            let modes = ffi::CString::new("r").unwrap();
            // Create the default classifier
            //

            let classifier: *const freesasa_classifier = &freesasa_protor_classifier;

            // Load file as C-style FILE pointer
            let pdb_file = fopen(pdb_filename.as_ptr(), modes.as_ptr());

            // Load structure
            let structure = freesasa_structure_from_pdb(pdb_file, classifier, 0);

            let fs_result = freesasa_calc_structure(structure, ptr::null());

            println!("Total SASA: {}", *(*fs_result).sasa);
        }
    }
}
