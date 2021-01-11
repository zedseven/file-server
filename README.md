# static_file_server
[![License: WTFPL](https://img.shields.io/badge/license-WTFPL-brightgreen.svg)](http://www.wtfpl.net/about/)

A basic static file server made with [Rocket](https://rocket.rs/) for serving up static files over HTTP.

## Usage
Usage is super simple: `file_server <path> [<port>]` (`port` is optional, and defaults to `8080`)

## Note
This isn't designed for production use, and moreso for quickly serving a directory of files for use on a local network.
In my case, I just wanted to be able to browse and watch local files on my PC, from my phone.
