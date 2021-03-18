# Static File Server
[![License: WTFPL](https://img.shields.io/badge/license-WTFPL-brightgreen.svg)](http://www.wtfpl.net/about/)

A basic file server made with [Rocket](https://rocket.rs/) for serving up static files over HTTP.

## Usage
Usage is super simple:
```file_server <path>```
(a different `port` can also be optionally specified, and defaults to `8080`)

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
