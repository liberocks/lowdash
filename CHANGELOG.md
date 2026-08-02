# Changelog

## [0.6.2] 2026-08-02
### Added
- Added Dependabot updates for Rust dependencies and GitHub Actions.

### Fixed
- Fixed `mean` for collections with more than 255 items.
- Fixed benchmark baseline and result artifact handling.
- Fixed DeepSource code coverage reporting.
- Updated GitHub Actions to avoid Node.js 20 deprecation warnings.

## [0.6.1] 2026-06-25
### Added
- Added broader benchmark coverage and edge-case tests.

### Changed
- Improved utility performance and CI coverage reporting.

## [0.6.0] 2026-06-25
### Added
- Added the expanded benchmark suite for the utility functions.

### Changed
- Improved benchmark and coverage workflows.

## [0.5.5] 2026-06-25
### Changed
- Improved performance in collection and string utilities.
- Improved test coverage for edge cases.

## [0.5.4] 2026-06-25
### Added
- Added benchmarks for the utility functions and benchmark CI.

### Changed
- Improved performance in several collection and string utilities.

## [0.5.3] 2025-02-03
### Changed
- Updated the README and package metadata.

## [0.5.2] 2025-02-03
### Changed
- Updated `permutation` to accept a parameter.
- Updated the README examples and documentation.

## [0.5.1] 2025-02-02
### Changed
- Updated documentation and package metadata.

## [0.5.0] 2025-02-02
### Added
- Added range, clamp, sum, product, mean, percentile, median, interpolation, permutation, combination, and duration utilities.

### Changed
- Improved CI configuration and removed generated benchmark reports from the repository.

## [0.4.0] 2024-12-22
### Added
- Added object utilities for assigning, selecting, mapping, transforming, and reading map entries and values.
- Added benchmark data for the find utilities.

## [0.3.0] 2024-12-02
### Added
- Added collection utilities for filtering, mapping, reducing, grouping, slicing, replacing, repeating, shuffling, and removing duplicates.
- Added sorting and counting utilities.

## [0.2.3] 2024-12-01
### Added
- Added the CI script for releasing the library to Cargo using GitHub Actions.

## [0.2.2] 2024-12-01
### Added
- Added automatic releases using GitHub Actions.

## [0.2.1] 2024-12-01
### Added
- Added the GitHub Actions release workflow.

## [0.2.0] 2024-12-01
### Added
- Added string utilities for casing, character length, chunking, truncation, random strings, substrings, and word splitting.
- Added `nearest_power_of_two` and expanded the project documentation.

## [0.1.0] 2024-11-30
### Added
- Initial release of the Lodash-inspired Rust utility library.
- Added the initial array and object utility functions, tests, and package metadata.
