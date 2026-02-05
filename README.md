# Let's Get Krabby

Ressources:
- [The Book](https://doc.rust-lang.org/book)

## Install 
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
rustup update
```
Or on Linux

```Bash
sudo apt install rustup
rustup default stable
```

## Doc 

```Bash
rustup doc # for all the doc available locally
rustup doc --book # e.g. for The Book
```
## Toolchain 

```Bash
rustc my_program.rs # Compile
rustfmt my_program.rs # Format in place 
cargo new hello_cargo # Create new project with cargo
cargo build # build the cargo project (to run from the dir)
cargo run # to run the project, it also recompiles if needed  
cargo check # to quickly check if the projects compiles without producin gthe executable
cargo clean # to clean the crate
cargo build --release # slower compile time but faster run time
cargo doc --open # to open the doc of the crates in dependencies
cargo fmt # to run to the auto formater
cargo test # c.f. Chapter 11
```

### Debug 

In VSCode install the extensionh `CodeLLDB` and `rust-analyzer`.

Then in `settings.json` specify path to Cargo.toml
```json
    "rust-analyzer.linkedProjects": [
        "${workspaceFolder}/Cargo.toml", // if the project is at the root 
        "${workspaceFolder}/[PATH]/Cargo.toml", // otherwise 
    ],
```
On the file click the debug button above the main function. 

We couldn't make the debuger pretty print with the usual `launch.json` VSCode debug.
 
### Auto format on save 

In `settings.json`
```Json
    "[rust]": {
            "editor.defaultFormatter": "rust-lang.rust-analyzer",
            "editor.formatOnSave": true
    },
```

### Specify `Cargo.toml` dir 

In `settings.json`
```Json
    "rust-analyzer.linkedProjects": [
            "${workspaceFolder}/Cargo.toml",
        ],
```
### Crate

A crate is the smallest amount of code that the Rust compiler considers at a time. This is something that can be compiled.

A crate can come in one of two forms: a binary crate (canb be executed, it has a main function) or a library crate (just implements functions). 

A package is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates. A package can contain as many binary crates as you like, but at most only one library crate

The root crate is `src/main.rs` for a binary crate and `src/lib.rs` for a library crate. A package can have multiple binary crates by placing files in the src/bin directory.

Needs to add under depencies in `Cargo.toml`.

[Repo](https://crates.io/)

## Owenrship & Borrowing

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### Fundamental borrowing rule

- if you have a mutable reference to a value, you can have no other references to that value.
- There can me multiple unmutable references.
- References must always be valid (dangling reference are not allowed).

### Scope 

A reference’s scope starts from where it is introduced and continues through the last time that reference is used.

Therefore a mutable reference scope must not overlap with any other reference scope.

### Slice

Slices let you reference a contiguous sequence of elements in a collection (i.e. array or tuple).