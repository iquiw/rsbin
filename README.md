# Rsbin: Rust Script Runner

## Overview

Rsbin compiles Rust or Haskell scripts to executables if necessary and executes the cached executables.

## Install

``` console
$ cargo install --git https://github.com/iquiw/rsbin
```

## Configuration

Configuration file is `~/.rsbin/config.toml` on Unix, `%APPDATA\rsbin\config.toml` on Windows.

``` toml
[[scripts]]
name = "foo"
path = "/path/to/foo.rs"
build-type = "rustc"

[[scripts]]
name = "bar"
path = "/path/to/bar.rs"
build-type = "Rustc"
```

## Usage

With the above configuration, run script `foo` as follows. It compiles `foo.rs` if not yet done and executes `foo` executable.

``` console
$ rsbin run foo
```

To list avaiable scripts, run `list`. With `-l` option, list paths also.

``` console
$ rsbin list
Available scripts:
  foo
  bar
  baz
```
