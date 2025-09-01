A basic program that plays with your screen brightness to notify you when your battery level reaches certain thresholds.

# Installation

1. Clone the repo
2. Inside the repo, run `cargo build`

An [AUR package](https://aur.archlinux.org/packages/batwatch-git) is also available for Arch Linux users.

A systemd service is also provided.

# Platform support

The program has only been tested on Linux, but since the [battery] and [brightness] crates also support Windows, the tool should, in principle, work there too.

[battery]: https://crates.io/crates/battery
[brightness]: https://crates.io/crates/brightness
