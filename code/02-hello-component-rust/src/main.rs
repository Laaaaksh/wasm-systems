// A wasi:cli "command" component. `cargo component` compiles this exactly
// like an ordinary Rust binary would target a normal OS - the difference is
// entirely in what target and what world it's built against (see the
// Makefile and `wit-world` note below). println! and std::env::args() work
// because the wasi:cli world wires them to WASI's stdout and cli/args
// imports - contrast with 01-core-wasm-no-wasi, where the same calls had
// no host interface to reach.
fn main() {
    let name = std::env::args().nth(1).unwrap_or_else(|| "world".to_string());
    println!("Hello, {name}!");
}
