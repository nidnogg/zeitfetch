## Contribution

If you feel like contributing to _zeitfetch_, feel free to fork it and open up a PR. Any merges will be checked for `cargo fmt` and `cargo clippy`.

So, before pushing changes to your branch, make sure you run:
```bash
# For checking formatting
cargo fmt --all --

# For linting
cargo clippy

```

To run the development environment, make sure you have both **Rust** and **Cargo** installed.
After that, in the root directory, run:

```bash
cargo run
```

Current priorities list:
* More distro ASCII Art in hex escape code format;
* Variable color configs for bolded text sections;
* Some refactoring here and there.
