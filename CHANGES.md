## [0.5.2] (2019-12-04)

- Re-export `SivAead` and `siv` module ([#43])

## [0.5.1] (2019-12-04)

- Remove crate type overrides ([#41])

## [0.5.0] (2019-12-04)

- Use `aes-siv` crate ([#36], [#39])
- Remove `soft-aes` feature ([#33])

**YANKED**: release's docs wouldn't build. Changed crate lib config.

## [0.4.2] (2019-01-12)

- Cargo.toml: Enable `soft-aes` feature in docs.rs metadata ([#16])

## [0.4.1] (2019-01-12)

- Cargo.toml: Fix docs.rs config, CI badge, and license string ([#13], [#14])

## [0.4.0] (2019-01-12)

- Add back (off-by-default) `soft-aes` feature ([#10])
- Convert benchmark suite to use criterion.rs ([#7])
- Refactor using `ctr` and `stream-cipher` crates ([#6])
- Update dependencies (closes #2) ([#4])
- Update to Rust 2018 edition ([#3])

## [0.3.0] (2017-12-25)

- STREAM support
- AEAD APIs: TypeScript, Rust
- Rust internals based on RustCrypto project providing ~10% faster performance

## [0.2.0] (2017-10-01)

- AES-PMAC-SIV support

# 0.1.0 (2017-07-31)

- Initial release

[0.5.2]: https://github.com/miscreant/miscreant.rs/pull/44
[#43]: https://github.com/miscreant/miscreant.rs/pull/43
[0.5.1]: https://github.com/miscreant/miscreant.rs/pull/42
[#41]: https://github.com/miscreant/miscreant.rs/pull/41
[0.5.0]: https://github.com/miscreant/miscreant.rs/pull/40
[#39]: https://github.com/miscreant/miscreant.rs/pull/39
[#36]: https://github.com/miscreant/miscreant.rs/pull/36
[#33]: https://github.com/miscreant/miscreant.rs/pull/33
[0.4.2]: https://github.com/miscreant/miscreant.rs/pull/17
[#16]: https://github.com/miscreant/miscreant.rs/pull/16
[0.4.1]: https://github.com/miscreant/miscreant.rs/pull/15
[#14]: https://github.com/miscreant/miscreant.rs/pull/14
[#13]: https://github.com/miscreant/miscreant.rs/pull/13
[0.4.0]: https://github.com/miscreant/miscreant.rs/pull/12
[#10]: https://github.com/miscreant/miscreant.rs/pull/12
[#7]: https://github.com/miscreant/miscreant.rs/pull/7
[#6]: https://github.com/miscreant/miscreant.rs/pull/6
[#4]: https://github.com/miscreant/miscreant.rs/pull/4
[#3]: https://github.com/miscreant/miscreant.rs/pull/3
[0.3.0]: https://github.com/miscreant/miscreant.rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/miscreant/miscreant.rs/compare/v0.1.0...v0.2.0
