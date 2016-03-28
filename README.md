# lazycell

<table>
    <tr>
        <td><strong>Linux / OS X</strong></td>
        <td><a href="https://travis-ci.org/indiv0/lazycell" title="Travis Build Status"><img src="https://travis-ci.org/indiv0/lazycell.svg?branch=master" alt="travis-badge"></img></a></td>
    </tr>
    <tr>
        <td colspan="2">
            <a href="https://indiv0.github.io/lazycell/lazycell" title="API Docs"><img src="https://img.shields.io/badge/API-docs-blue.svg" alt="api-docs-badge"></img></a>
            <a href="https://crates.io/crates/lazycell" title="Crates.io"><img src="https://img.shields.io/crates/v/lazycell.svg" alt="crates-io"></img></a>
            <a href="#License" title="License: MIT/Apache-2.0"><img src="https://img.shields.io/crates/l/lazycell.svg" alt="license-badge"></img></a>
            <a href="http://clippy.bashy.io/github/indiv0/lazycell/master/log" title="Clippy Linting Result"><img src="http://clippy.bashy.io/github/indiv0/lazycell/master/badge.svg" alt="clippy-lint-badge"></img></a>
        </td>
    </tr>
</table>

Rust library providing a lazily filled Cell.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
lazycell = "0.1"
```

And in your `lib.rs` or `main.rs`:

```rust
extern crate lazycell;
```

See the [API docs][api-docs] for information on using the crate in your library.

## Credits

The LazyCell library is based originally on work by The Rust Project Developers
for the project [crates.io][crates-io-repo].

## License

LazyCell is distributed under the terms of both the MIT license and the Apache
License (Version 2.0).

See [LICENSE-APACHE][license-apache], and [LICENSE-MIT][license-mit] for details.

[api-docs]: https://indiv0.github.io/lazycell/lazycell
[crates-io-repo]: https://github.com/rust-lang/crates.io "rust-lang/crates.io: Source code for crates.io"
[license-apache]: https://github.com/indiv0/lazycell/blob/master/LICENSE-APACHE "Apache-2.0 License"
[license-mit]: https://github.com/indiv0/lazycell/blob/master/LICENSE-MIT "MIT License"
