# kicad_files ![License: Apache-2.0 OR LGPL-3.0](https://img.shields.io/badge/license-Apache--2.0%20OR%20LGPL--3.0-blue) [![kicad_files on crates.io](https://img.shields.io/crates/v/kicad_files)](https://crates.io/crates/kicad_files) [![kicad_files on docs.rs](https://docs.rs/kicad_files/badge.svg)](https://docs.rs/kicad_files) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/kicad-rs/kicad_files)

A library to read KiCAD v6 file formats.

**This crate is not affiliated with KiCAD.**

Currently, the following file formats are supported:

 - KiCAD v5 Footprint file (`*.kicad_mod`), starting with `(module`
 - KiCAD v6 Footprint file (`*.kicad_mod`), starting with `(footprint`
 - KiCAD v6 Symbol library (`*.kicad_sym`), starting with `(kicad_symbol_lib`

