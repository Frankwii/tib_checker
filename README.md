# TIB checker

Checks whether the train schedule has changed and sends a notification if so.

## Dependencies
+ Rust tooling
+ Sqlite

## Installation
Run the following in the root of the project
```sh
cargo build --release
```

Your binary should be `target/release/tib_checker`. Do what you want with it. I symlink mine to somewhere in my PATH and have added a systemd to automatically execute it every day.
