# Messible Interface

The Messible is an Ansible-like device that allows you to send small messages from an embedded device to a controlling device.  It's mostly useful for printing debug messages and panics.

## Usage

Add the crate to your Cargo.toml.  Then import the printer functions:

```rust
#![no_std]
#![no_main]

use messible::mprintln

#[entry]
fn main() -> ! {
    mprintln!("Hello, world!");
    loop {};
}
```

The Messible defaults to an offset of `0xe000_7800`.  If you're running on a system with it at a different offset, call `messible::set_address(new_addr)`.  Note that this is an `unsafe` function.

## Contribution Guidelines

[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg)](CODE_OF_CONDUCT.md)

Please see [CONTRIBUTING](CONTRIBUTING.md) for details on
how to make a contribution.

Please note that this project is released with a
[Contributor Code of Conduct](CODE_OF_CONDUCT.md).
By participating in this project you agree to abide its terms.

## License

Copyright © 2019

Licensed under the [Apache License 2.0](http://opensource.org/licenses/Apache-2.0) [LICENSE](LICENSE)
