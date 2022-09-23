#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::pedantic, clippy::nursery)]

/*!
## Requirements

This crate requires a nightly compiler.

## Whats next?

* [ ] Update crate name
  - [ ] `README.md` symlink
  - [ ] `<crate_name>/Cargo.toml` update keys
* [ ] Update `README.md` (Badges and text)
* [ ] Copy `README.md` to `<create_name>/src/lib.rs`
* [ ] Search for `TODO`, `raldone01`, `rust_nightly_crate_rs` and `rust_nightly_crate_rs_template`

   (<kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>F</kbd> in vscode)
* [ ] Tag releases with `v<d>.<d>.<d>` and push the tag.
* [ ] Run `cargo publish` to publish to `crates.io`.
* [ ] Remove this section

## What can this crate do?

TODO: Crate description here

## Example

```rust
use rust_nightly_crate_rs::hello_world;

// TODO: Update this example
println!("{}", hello_world());
```

## Authors

[raldone01](https://github.com/raldone01) and [onestacked](https://github.com/chriss0612) are the primary authors and maintainers of this library.

## License

This project is released under either:

- [MIT License](https://github.com/raldone01/const_sort_rs/blob/main/LICENSE-MIT)
- [Apache License (Version 2.0)](https://github.com/raldone01/const_sort_rs/blob/main/LICENSE-APACHE)

at your choosing.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
*/

mod hello;
pub use hello::*;

#[cfg(test)]
mod test;
