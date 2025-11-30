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

### Crate

Collection of rust code meant to be added to our source code.

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