# CodeWars-rust-cgdk
[![Build Status](https://travis-ci.org/elsid/CodeWars-rust-cgdk.svg?branch=master)](https://travis-ci.org/elsid/CodeWars-rust-cgdk)
[![Build status](https://ci.appveyor.com/api/projects/status/6ltjv637jjvf382e/branch/master?svg=true)](https://ci.appveyor.com/project/elsid/codewars-rust-cgdk/branch/master)

Rust language package for CodeWars - [Russian AI Cup 2017](http://russianaicup.ru)

## Build

For linux, windows and macos:
```
cargo build
```

or for linux and macos:
```bash
./compile-rust.sh
```

or for windows:
```
compile-rust.bat
```

## Usage

Write your own strategy in [src/my_strategy.rs](src/my_strategy.rs).

To import modules into my_strategy.rs use path attribute:
```rust
#[path = "some_module.rs"]
mod some_module;

pub struct MyStrategy {}
```

Run binary with exactly 3 arguments (host, port, token) or without arguments to use defaults:
```bash
./MyStrategy 127.0.0.1 31001 0000000000000000
./MyStrategy
```
