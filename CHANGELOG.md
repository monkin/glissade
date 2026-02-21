# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Update `glam` dependency to 0.31 in `poly` example.

## [0.2.9] - 2026-02-01

### Changed

- Updated `glam` dependency to 0.31.
- Updated Material Design Lite (MDL) location in examples.

## [0.2.8] - 2025-12-30

### Added

- Added `web-time` feature to `glissade` dependencies in examples.

### Changed

- Adjusted default features (`web-time` is not default anymore).
- Updated README files.

## [0.2.7] - 2025-10-12

### Changed

- Updated dependencies.
- Fixed dependency issues in examples.

## [0.2.6] - 2025-03-09

### Changed

- Updated `glam` dependency to 0.30.

## [0.2.5] - 2024-10-31

### Fixed

- Fixed keyframes scaling logic.

## [0.2.4] - 2024-10-30

### Added

- Added `keyframes::map` function for transforming keyframe values.

## [0.2.3] - 2024-10-30

### Added

- Added `Keyframes` implementations for tuples of keyframes, allowing synchronized animations.
- Added `ApplyEasingKeyframes` for applying easing to existing keyframes.

## [0.2.2] - 2024-09-01

### Removed

- Removed `TimeDiff` trait in favor of standard duration handling.

### Added

- Added `keyframes::function` documentation.

## [0.2.1] - 2024-09-01

### Changed

- Moved keyframe constructors into the `keyframes` module.
- Updated `keyframes` module documentation.

## [0.2.0] - 2024-09-01

### Added

- Added `FunctionKeyframes` to support creating keyframes from functions/closures.

### Changed

- Made `is_finite` check mandatory for keyframes to improve robustness.
- Removed `Mix` restriction from several keyframe functions.
- Updated example and pipeline dependencies.

## [0.1.42] - 2024-08-31

### Added

- Added `keyframes::slice` method for taking a portion of an animation.

### Changed

- Reduced and removed several trait restrictions (like `PartialEq`) for keyframes.
- Fixed "Go to sources" link in the `poly` example.

## [0.1.41] - 2024-08-29

### Added

- Added `Animated::flatten` method.

### Fixed

- Fixed tabular easing creation example.

## [0.1.40] - 2024-08-27

### Added

- Added `Animated::join` method for combining animated values.
- Added tests for `Animated::join`.

### Changed

- Added descriptive comments to the `Distance` trait.
- Prevented `poly` example from being published.

## [0.1.39] - 2024-08-23

### Changed

- Added `Distance` trait description to the README.

## [0.1.38] - 2024-08-22

### Added

- Added animation along a path example.
- Added composed path implementation.
- Added `Poly` implementation and removed old bezier-related code.
- Added `poly` example build to the pipeline.

### Fixed

- Fixed a crash in `RepeatKeyframes`.

## [0.1.37] - 2024-08-13

### Added

- Added `PathBuilder` for easier path construction.
- Added support for bezier curves.
- Added `Distance` trait and implementations for external library types.

### Changed

- Renamed `Curve::get` to `value_at`.
- Refactored keyframes into separate files.
- Used `SmoothArray` for tabular easing.

### Fixed

- Fixed an issue in `RepeatKeyframes`.

## [0.1.36] - 2024-07-27

### Added

- Added `glam` support.
- Added `glam` feature description to README.

### Changed

- Removed 0..1 clamping for time `t` to allow extrapolation.
- Renamed time implementation files and moved external implementations to a separate directory.
- Fixed typos in README.

## [0.1.35] - 2024-07-26

### Added

- Added `Animated` trait usage example.

## [0.1.34] - 2024-07-24

### Added

- Implemented `Stationary` for `palette`, `nalgebra`, `cgmath`, and `euclid` types.

### Changed

- Updated `.gitignore`.
- Cleaned up feature documentation.

## [0.1.33] - 2024-07-24

### Added

- Added `Animated::map` function.
- Added `--all-features` support for `docs.rs`.

## [0.1.32] - 2024-07-24

### Added

- Added `Animated` implementation for tuples and arrays of `Animated` items.
- Added `Stationary` description to README.

### Changed

- Updated keyframe item restrictions and removed redundant `Sized` constraints.
- Fixed several typos in comments and README.

## [0.1.31] - 2024-07-23

### Added

- Added support for `Stationary` values.

## [0.1.30] - 2024-07-23

### Changed

- Updated `Transition` comments in keyframes.
- Improved `follow-cursor` example startup behavior.

## [0.1.29] - 2024-07-23

### Added

- Introduced the `Animated` trait as a common interface for `Animation` and `Inertial`.

### Removed

- Removed `AnimatedItem` trait in favor of the new `Animated` trait.

## [0.1.28] - 2024-07-23

### Changed

- Updated README examples.

## [0.1.27] - 2024-07-22

### Changed

- Renamed `InertialValue` to `Inertial`.
- Updated README and disabled running examples from it.

## [0.1.26] - 2024-07-22

### Added

- Added `follow-cursor` and `console-inertial` examples.
- Added live example links and source buttons to examples.

### Changed

- Renamed `inertial` demo to `shape-animation`.
- Improved library documentation by including README content.

## [0.1.25] - 2024-07-21

### Fixed

- Fixed zero duration transitions for `Inertial`.

## [0.1.24] - 2024-07-20

### Changed

- Replaced `std::time::Instant` with a custom `Time` trait to support varied time sources.

## [0.1.23] - 2024-07-19

### Added

- Implemented `From<T>` for `Inertial`.

### Changed

- Renamed `Transition` to `Keyframes`.
- Made `Debug` implementation optional for some types.
- Updated README with documentation links.

### Fixed

- Fixed `Option<Mix>` interpolation logic.

## [0.1.22] - 2024-07-19

### Added

- Implemented `Default` for `Inertial`.

### Changed

- Updated README.

## [0.1.21] - 2024-07-18

### Changed

- Switched from `SystemTime` to `Instant` for animations.
- Updated animation examples and links in the README.

## [0.1.20] - 2024-07-18

### Added

- Added `transition` example.
- Added source links to the `inertial` example.

### Changed

- Simplified JavaScript imports in examples.
- Initialized Material Design Lite (MDL) components when rendered.

## [0.1.19] - 2024-07-17

### Added

- Added `InertialValue` example.
- Added GitHub Pages workflow for examples.
- Added live example links to the README.

### Fixed

- Fixed tests for `Inertial`.
- Fixed font links in examples.

## [0.1.18] - 2024-07-14

### Added

- Integrated `web-time` for WASM compatibility.

## [0.1.17] - 2024-07-12

### Added

- Added support for inverted transitions.

## [0.1.16] - 2024-06-26

### Added

- Implemented `Mix` for slices.
- Added license link to the README.

### Changed

- Updated dependencies and README.

## [0.1.15] - 2024-06-26

### Fixed

- Fixed quaternion tests and formatting.

## [0.1.14] - 2024-06-26

### Added

- Added `cgmath` support.

## [0.1.13] - 2024-06-24

### Changed

- README updates.

## [0.1.12] - 2024-06-24

### Fixed

- Fixed doctests when the `derive` feature is absent.
- Dependency and formatting fixes.

## [0.1.11] - 2024-06-24

### Added

- Added `Mix` derive macro for easy implementation on custom structs.

## [0.1.10] - 2024-06-22

### Added

- Added feature information to the README.

### Changed

- Renamed `linear_to` to `go_to` for better clarity.

### Fixed

- Fixed `palette` dependency version.

## [0.1.9] - 2024-06-22

### Added

- Added support for `palette` colors.
- Improved matrix support for `nalgebra`.

## [0.1.8] - 2024-06-22

### Changed

- Updated README with references to `euclid` and `nalgebra`.

## [0.1.7] - 2024-06-22

### Added

- Added `nalgebra` and `euclid` support.

### Changed

- Updated tabular easing documentation.

## [0.1.6] - 2024-06-21

### Added

- Added bezier images to the documentation.

### Fixed

- Fixed bezier easing issues.

## [0.1.5] - 2024-06-20

### Added

- Added steps easing images to the documentation.

## [0.1.4] - 2024-06-20

### Added

- Added easing images to the documentation.

### Removed

- Removed `i128`/`u128` `Mix` implementations due to limited utility and potential issues.

## [0.1.3] - 2024-06-20

### Added

- Added `end_time` and `target` properties to `Inertial`.
- Added GitHub Actions workflows.
- Added badges to the README.

### Fixed

- Fixed several clippy linting issues.

## [0.1.2] - 2024-06-20

### Changed

- README updates.

## [0.1.1] - 2024-06-20

### Added

- Added project description and usage examples to the README.

## [0.1.0] - 2024-06-20

### Added

- Initial release with core animation traits and structures.
- Support for various easing functions and keyframe types.
