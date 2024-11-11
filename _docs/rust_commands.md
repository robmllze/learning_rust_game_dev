# Rust Command Basics

## Create a new project

```sh
cargo new you_project_name
```

## Upgrade Rust

```sh
rustup self update && rustup update stable
```

## Building

```sh
cargo build
cargo build --release
# or
rustc main.rs
```

## Building and running

```sh
rm -rf target
cargo run
```
