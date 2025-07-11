# pxlfmt

Type-safe pixel formats and pixel manipulation.

[![Test](https://github.com/crates-lurey-io/pxlfmt/actions/workflows/test.yml/badge.svg)](https://github.com/crates-lurey-io/pxlfmt/actions/workflows/test.yml)
[![Crates.io Version](https://img.shields.io/crates/v/pxlfmt)](https://crates.io/crates/pxlfmt)
[![codecov](https://codecov.io/gh/crates-lurey-io/pxlfmt/graph/badge.svg?token=Z3VUWA3WYY)](https://codecov.io/gh/crates-lurey-io/pxlfmt)

## Contributing

This project uses [`just`][] to run commands the same way as the CI:

- `cargo just check` to check formatting and lints.
- `cargo just coverage` to generate and preview code coverage.
- `cargo just doc` to generate and preview docs.
- `cargo just test` to run tests.

[`just`]: https://crates.io/crates/just

For a full list of commands, see the [`Justfile`](./Justfile).

## Inspiration

- [`@thi.ng/pixel`](https://github.com/thi-ng/umbrella/tree/develop/packages/pixel)
- [`pixel_formats`](https://crates.io/crates/pixel_formats)
- [`pixelfmt`](https://crates.io/crates/pixelfmt)
- [`libuv`](https://chromium.googlesource.com/libyuv/libyuv)
