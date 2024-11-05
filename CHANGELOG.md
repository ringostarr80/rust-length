# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.2.3] - 2024-11-05
### Added
- badge for CodeCoverage to README.md

### Changed
- renamed CHANGELOG to CHANGELOG.md
- updated cargo dependencies

## [0.2.2] - 2022-11-28
### Added
- new SI-units: ronna, quetta, ronto, quecto from https://www.npl.co.uk/si-prefix

## [0.2.1] - 2022-07-15
### Changed
- updated dependencies and rust edition to 2021

## [0.2.0] - 2019-10-20
### Added
- LICENSE and this CHANGELOG file
- new methods: add(&self, length: Length) -> Self; add_by_ref(&mut self, length: Length) -> &mut Self
- new methods: subtract(&self, length: Length) -> Self; subtract_by_ref(&mut self, length: Length) -> &mut Self
- new methods: multiply_by<T: Into<f64>>(&self, factor: T) -> Self; multiply_by_ref<T: Into<f64>>(&mut self, factor: T) -> &mut Self
- new methods: divide_by<T: Into<f64>>(&self, factor: T) -> Self; divide_by_ref<T: Into<f64>>(&mut self, factor: T) -> &mut Self
- new methods: normalize(&self) -> Self; normalize_by_ref(&mut self) -> &mut Self

## [0.1.0] - 2019-10-04
### Added
- initial Release

[unreleased]: https://github.com/ringostarr80/rust-length/compare/v0.2.3...HEAD
[0.2.3]: https://github.com/ringostarr80/rust-length/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/ringostarr80/rust-length/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/ringostarr80/rust-length/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/ringostarr80/rust-length/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ringostarr80/rust-length/releases/tag/v0.1.0