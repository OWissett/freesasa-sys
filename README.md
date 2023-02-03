# FreeSASA FFI

This crate provides raw FFI bindings to the [freesasa](https://freesasa.github.io/doxygen/index.html)
C library, developed by [Simon Mittinatten](https://github.com/mittinatten) \[1\]. FreeSASA allows
you to calculate the solvent accessible surface area (SASA) of a protein from its atomic
coordinates. The library is written in C, and is available under the MIT license.

The bindings in this crate are automatically generated by `bindgen`. Since this crate provides
raw access to a C-library, it is on the user to ensure that memory safety is maintained.

For most uses it is recommended to use the RustSASA crate (which is currently not publicly available).
RustSASA aims to provide a safe idiomatic way of working with freesasa from within Rust.

Please contact me via email, ow257@cam.ac.uk, if you are interested in access to RustSASA.

## License
The MIT license