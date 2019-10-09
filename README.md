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
