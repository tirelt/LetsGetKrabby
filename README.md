# Let's Get Krabby

## Install 
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
rustup update
```

## Toolchain 

```Bash
rustc my_program.rs # Compile
rustfmt my_program.rs # Format in place 
cargo new hello_cargo # Create new project with cargo
cargo build # build the cargo project (to run from the dir)
cargo run # to run the project, it also recompiles if needed  
cargo check # to quickly check if the projects compiles without producin gthe executable
cargo build --release # slower compile time but faster run time
cargo doc --open # to open the doc of the crates in dependencies
```
### Crate

Collection of rust code meant to be added to our source code.

Needs to add under depencies in `Cargo.toml`.


