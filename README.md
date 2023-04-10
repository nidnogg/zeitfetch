<div align="center">
<h1> zeitfetch 📸 </h1>

Instantaneous snapshots of system information <br />
Linux • macOS • Windows <br />

![License](https://img.shields.io/github/license/nidnogg/zeitfetch?style=for-the-badge)
![Size](https://img.shields.io/github/repo-size/nidnogg/zeitfetch?color=orange&logo=rust&style=for-the-badge)
![Stars](https://img.shields.io/github/stars/nidnogg/zeitfetch?color=red&style=for-the-badge)
</div>

## About

[_zeitfetch_](https://crates.io/crates/zeitfetch) is a [neofetch](https://github.com/dylanaraps/neofetch) implementation in Rust aiming to be near instantaneous, with the least amount of dependencies possible.

Considering the shortage of fully cross platform screenfetch solutions (including Windows shells) this crate aims to fill that gap.

It's in early stages at the moment, and support for Windows 10, Windows 11, Mac OS X, Debian, Ubuntu, Fedora and Arch is implemented for the time being, with more on the way.

If your Linux distribution is not listed/not being actively tested on, most likely it will be missing a dedicated OS logo, so an ASCII rendition of [Tux](https://en.wikipedia.org/wiki/Tux_(mascot)) will be displayed instead.


## Features

Fully cross platform compatible with Windows, macOS, and most Linux distros.

## Installation

### Using Cargo

With **Cargo** installed, run:

```bash
cargo install zeitfetch
```
### Using Homebrew
On macOS, *zeitfetch* can also be installed via **brew**:

```bash
brew tap nidnogg/zeitfetch
brew install zeitfetch
```
## Note for running on terminal boot

 Some people like to run fetch programs as soon as any terminal tab is open (e.g: from within `.bashrc`).

 For this, within Operating Systems other than macOS, Rust binaries have to be reloaded to the PATH environment variables before running the desired command. When editing out a `.bashrc`, `.zshrc` file or whichever is available, adding the following line **before** _zeitfetch_ will ensure that it works correctly:

 ```bash
export PATH="$HOME/.cargo/bin:$PATH"
```
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
## Disclaimer for ASCII logos

Most of the ASCII art from _zeitfetch_ is not originally made by me, only slightly modified here and there. All of the respective credit for them is listed below:

| OS | Author | Source
| --- | --- | --- |
| Debian | [dylanaraps](https://github.com/dylanaraps) | [neofetch](https://github.com/dylanaraps/neofetch) |
| Ubuntu | [KittyKatt](https://github.com/KittyKatt) | [screenfetch](https://github.com/KittyKatt/screenFetch) |
| Fedora | likw1dus | [pagure.io](https://pagure.io/design/issue/736) |
| Arch | [trizen](https://aur.archlinux.org/packages/alsi/) | [wiki.archlinux.org (alsi)](https://wiki.archlinux.org/title/ASCII_art) |
| MacOS | [shelldandy](https://github.com/shelldandy) | [neofetch PR](https://github.com/dylanaraps/neofetch/issues/789) |
| Windows 11 | [kiedtl](https://github.com/kiedtl) | [winfetch](https://github.com/kiedtl/winfetch) |
| Windows 10 | [HotDog640](https://github.com/HotDog640) | [neofetch PR](https://github.com/dylanaraps/neofetch/issues/1466) |
| Linux | Joan Stark (jgs) | [Wikipedia](https://en.wikipedia.org/wiki/Joan_Stark) |

If any artists feel like this is an issue in particular, please feel free to drop me an email @ [henriquevt98@gmail.com](mailto:henriquevt98@gmail.com) and I'll remove any infringing works ASAP.

## Latest Updates

Check out the [release notes](https://github.com/nidnogg/zeitfetch/releases)!