# File Server
[![License: WTFPL](https://img.shields.io/badge/license-WTFPL-blue.svg)](http://www.wtfpl.net/about/)

A basic file server made with [Rocket](https://rocket.rs/) for serving up static files over HTTP.

## Usage
Usage is super simple:
```
file_server <path>
```
(a different `port` can also be optionally specified, and defaults to `8080`)

Then just access the device running `file_server` at `<ip>:8080/<file_path>`.

## Installation
If you have Rust installed on your machine, simply run:
```
cargo install --git https://github.com/zedseven/file-server
```

If not, you should likely be able to get a working executable for your platform from the Releases tab.

## Building
Run the following commands from the project directory.

Because the program uses Rocket, you must use the nightly build of Rust. To set Rust to use nightly for just this project,
use the following command:
```
rustup override set nightly
```
Then, just build the executable as normal:
```
cargo build --release
```
The built executable will be in `target/release`.

## Note
This isn't designed for production use, and moreso for quickly serving a directory of files for use on a local network.
In my case, I just wanted to be able to browse and watch local files on my PC, from my phone.
