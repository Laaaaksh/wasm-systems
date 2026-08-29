// Targets the exact same `../wit/world.wit` as the Python and Go
// implementations in this directory (see Cargo.toml's
// [package.metadata.component.target]). Same WASI CLI contract, three
// unrelated toolchains, one guest-side program each.
fn main() {
    let name = std::env::args().nth(1).unwrap_or_else(|| "world".to_string());
    println!("Hello, {name}, from Rust!");
}
