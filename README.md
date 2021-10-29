
# notes_rust

A simple cli for taking daily notes, written in Rust.

## About

Use this tool to quickly take notes from the command line. By default, this program
pulls up a note specific for the current date. Check out [the docs](./docs/tutorial.txt)
to see to use it.

## Installation

Currently, only installation via [cargo](https://github.com/rust-lang/cargo) is supported.

### via Cargo

If you already have cargo installed, just you can run:

```bash
cargo install --git https://github.com/samsonjj/notes_rust
notes_rust --help
```

Or you can clone the source and compile it:

```bash
git clone https://github.com/samsonjj/notes_rust
cargo build --release
./target/release/notes_rust --help
```

## How to use

I highly recommend setting an alias in your .bash_profile or equivalent:

```bash
echo "alias notes=notes_rust" >> ~/.bash_profile
```

Check out the [tutorial docs](docs/tutorial.txt) or install it and run

```
notes_rust --tutorial
```

or

```
notes_rust --help
```
