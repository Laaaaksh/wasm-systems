// The exact same source, compiled to two different targets by the
// Makefile in this directory. That's the whole point of this sample:
// nothing about *this code* changes between the two builds. What
// changes is whether the compiler has an OS interface to compile
// `println!` against.
fn main() {
    let sum = 2 + 3;
    println!("sum = {sum}");
}
