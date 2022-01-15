A quick experiment to crosscompile a basic rust project with C dependencies

```sh
# setup
sudo apt install llvm-dev libclang-dev clang mingw-w64
rustup target add x86_64-pc-windows-gnu

# compile
cargo build --target x86_64-pc-windows-gnu
```
