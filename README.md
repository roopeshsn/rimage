# Rimage

[![Build Status](https://img.shields.io/github/actions/workflow/status/SalOne22/rimage/rimage.yml?label=rimage&style=flat-square)](https://github.com/SalOne22/rimage/actions?query=branch%3Amain+)
[![docs.rs](https://img.shields.io/docsrs/rimage/latest?style=flat-square)](https://docs.rs/rimage)
[![Version](https://img.shields.io/crates/v/rimage?style=flat-square)](https://crates.io/crates/rimage)
[![License](https://img.shields.io/crates/l/rimage?style=flat-square)](https://github.com/SalOne22/rimage)

This is CLI tool inspired by [squoosh!](https://squoosh.app/)  
Rimage currently supports three codecs, mozjpeg, oxipng and webp, and aims to add support for AVIF and JPEG XL in the future.

## Installation

If you have cargo you can use this

```
cargo install rimage
```

or from [Releases](https://github.com/SalOne22/rimage/releases) on GitHub

## Usage

```
rimage -q 75 *.jpg
```

- Quality: `-q 0` through `-q 100`, higher is better
- Output format: `-o png`, currently supported only jpg, png, oxipng and webp
- Suffix for output: `-s _updated`, adds suffix in file name ("input.jpg" -> "input_updated.jpg")
- Info: `-i`, flag used to get info about images (size and data length)
- Threads: `-t 4`, number of threads to use
- Quantization: `--quantization 50`, quality of quantization from 0 to 100, higher is better
- Dithering: `--dithering 0.5`, quality of dithering from 0 to 1, higher is better
- Resize: `--width 250` or `--height 100`, resizes image to specified width or height
- Filter: `--filter mitchell`, filter used to resizing
- Logging: `RUST_LOG=trace`, enables logging output, more information see [here](https://docs.rs/env_logger/latest/env_logger/)
- More options will be added later

## To-Do

- Support for AVIF and JPEG XL
- And allot of bugfixes and optimizations

## Changelog

Read changelog [here](CHANGELOG.md)

## Development

Please read the [contribution guide](CONTRIBUTING.md)

## License

Rimage is licensed under either the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0), or [the MIT license](https://opensource.org/licenses/MIT).

All images are taken from [PNGSuite](http://www.schaik.com/pngsuite/)

## Contribute

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
