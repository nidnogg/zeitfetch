<div align="center">
<h1> zeitfetch 📸 </h1>

Instantaneous snapshots of system information <br />
Linux • macOS • Windows <br />
    
![License](https://img.shields.io/github/license/nidnogg/zeitfetch?style=for-the-badge)
![Size](https://img.shields.io/github/repo-size/nidnogg/zeitfetch?color=orange&logo=rust&style=for-the-badge)
![Stars](https://img.shields.io/github/stars/nidnogg/zeitfetch?color=red&style=for-the-badge)
</div>

## About

_zeitfetch_ is a [neofetch](https://github.com/dylanaraps/neofetch) implementation in Rust aiming to be near instantaneous, with the least amount of dependencies possible.

Considering the shortage of fully cross platform screenfetch solutions (including Windows shells) this crate aims to fill that gap.

It's in early stages at the moment, and support for Windows 10, Windows 11, Mac OS X, Debian, Ubuntu and Fedora is implemented for the time being, with more on the way. 

Other distros/BSD should work fine, but there should be a lot of ASCII art missing. A placeholer Linux/BSD ASCII logo will be added for those systems not being actively worked on.

## Features

Fully cross platform compatible with Windows, macOS (almost there), and most Linux distros.

## Installation

For the time being, since neither a crate nor a scoop package have been published yet, an executable titled **zeitfetch** can be created under the folder `target/release` by running `cargo run` in the root directory. This assumes you have both **Rust** and **Cargo** installed.
## Contribution 

If you feel like contributing to _zeitfetch_, feel free to fork it and open up a PR. There are no particular coding guidelines so any improvements you can think of are fine.

Current to-be-implemented list:
* Proper macOS implementation (I'm off my Mac system for a while);
* CPU and GPU information (not working from sys_info, apparently);
* More distro ASCII Art in hex escape code format.
## Disclaimer for ASCII logos

Most of the ASCII art from _zeitfetch_ is not originally made by me, only slightly modified here and there. All credits for them go to:

| OS | Author | Source
| --- | --- | --- |
| Windows 11 | [kiedtl](https://github.com/kiedtl) | [winfetch](https://github.com/kiedtl/winfetch) |
| Windows 10 | [HotDog640](https://github.com/HotDog640) | [neofetch PR](https://github.com/dylanaraps/neofetch/issues/1466) |
| Debian | [dylanaraps](https://github.com/dylanaraps) | [neofetch](https://github.com/dylanaraps/neofetch) |
| Ubuntu | [KittyKatt](https://github.com/KittyKatt) | [screenfetch](https://github.com/KittyKatt/screenFetch) |
| Fedora | likw1dus | [pagure.io](https://pagure.io/design/issue/736) |

If any artists feel like this is an issue in particular, please feel free to drop me an email @ [henriquevt98@gmail.com](mailto:henriquevt98@gmail.com) and I'll remove any infringing works ASAP.

