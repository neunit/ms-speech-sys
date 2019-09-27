# CHANGELOG

## [0.2.0] - 2019-9-27

### Fixed

- Bump version to 0.2.0.
- Change name from "ms_speech_sys" to "ms-speech-sys".
- Change rustfmt configuration according to rust project and apply it.
- Macro DeriveHandle add support for member destructure function.


## [0.1.2] - 2019-7-24

### Fixed

- To avoid UB, initialize the buffer in macro `ffi_get_string`.

## [0.1.1] - 2019-7-24

### Changed

- Add error handling and macros.
- In order to comply with the rule of crate.io, decrease keywords to 5 in Cargo.toml.
- Add some helper functions.
- Add `properties` mod.

## [0.1.0] - 2019-6-28

### Added

- Low level and simple wrap of Microsoft Speech SDK with bindgen automation.
