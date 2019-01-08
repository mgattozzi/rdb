# rdb

Stop futzing with weird Rust Debugger corner cases and start debugging your code!

## Supported Debuggers

Currently only `LLDB`.

## Dependencies

- Rust installed via `rustup`
- `LLDB`

## Getting Started

```bash
cargo install rdb
```

Build your code once in debug:

```bash
cargo build
```

From the directory where `target` exists:

```bash
rdb PROGRAM_NAME
```

Or from somewhere else:

```bash
rdb PROGRAM_NAME PATH_TO_DIR_CONTAINING_TARGET
```

Debug away!

## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for more information.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
