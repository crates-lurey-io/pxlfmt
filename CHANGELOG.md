# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Documented that `U32x8888` is a `#[repr(transparent)]` over a `u32`.

### Changed

- `Pixel::from` was removed in favor of `Pixel::new`, which is now `impl RawPixel`.

## [0.1.0] - 2025-07-10

### Added

- Initial release.
