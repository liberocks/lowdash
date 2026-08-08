# Changelog

## [0.8.0] 2026-08-08
### Added
- Added `take` for selecting the first items from a collection.
- Added `take_right` for selecting the final items from a collection.
- Added `find_last` for reverse predicate searches.
- Added `chunk_by` for grouping adjacent runs by key.
- Added `scan` for collecting intermediate accumulators.
- Added `assign_with` for resolver-based map merging.
- Added `difference_by` for key-based collection differences.
- Added `intersection_by` for key-based collection intersections.
- Added `zip_longest` for pairing collections of different lengths.
- Added `cartesian_product` for left-major pair generation.
- Added `union_by` for key-based collection unions.
- Added `symmetric_difference` for stable exclusive values.
- Added `unzip` for splitting pairs into two collections.
- Added `min_by_key` for selecting the first minimum keyed item.
- Added `max_by_key` for selecting the first maximum keyed item.
- Added `defaults` for first-value-wins map merging.
- Added `invert_grouped` for grouping all keys by value.
- Added `windows` for producing overlapping collection windows.
- Added `modes` for finding most frequent values.
- Added `variance` for stable population variance calculations.
- Added `parallel_map` for ordered concurrent mapping.
- Added `parallel_try_map` for ordered concurrent fallible mapping.
- Added `parallel_for_each` for concurrent side-effect callbacks.
- Added `parallel_find_map` for ordered concurrent searches.
- Added `parallel_reduce` for chunked concurrent reductions.
- Added `floating_mean` for compensated floating-point means.
- Added `weighted_mean` for validated weighted averages.

## [0.7.0] 2026-08-02
### Added
- Added stable `sort_by` and `sort_by_key` collection sorting functions.
- Added `difference`, `intersection`, and `union` collection set functions.
- Added `take_while` and `take_right_while` prefix and suffix functions.
- Added `zip` for pairing items from two collections.
- Added `every` and `some` short-circuiting predicate functions.
- Added unit tests, README documentation, and Criterion benchmarks for all new functions.

### Changed
- Updated coverage workflow triggers so feature branches are covered.
- Switched coverage reporting from Tarpaulin to LLVM coverage for more accurate DeepSource results.
- Improved `assign` by 45.9% to 46.8% through map capacity reservation.
- Improved `chunk` by 15.6% to 26.6% through outer vector capacity reservation.
- Improved `combination` by 17.4% to 36.9% by removing intermediate clones.
- Improved `count_values` by 43.0% to 57.9% by cloning keys only once per distinct value.
- Improved `filter_map` by 25.4% to 29.7% through result capacity reservation.
- Improved `keys` by 13.8% to 21.4% through exact output capacity reservation.
- Improved `map_keys` by 30.1% to 35.4% by avoiding repeated map lookups.
- Improved `uniq_keys` by 35.5% to 37.9% by avoiding duplicate key clones.
- Simplified README headings by removing decorative emoji.

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
