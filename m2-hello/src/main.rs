//! Provable contract: commutativity, associativity, additive identity.
//! A `cargo run` that exits zero is the proof.
//!
//! All assertions live in `m2_hello::run`, which is the testable seam.
//! `main` is the one-line shell that prints the success marker.

fn main() {
    println!("{}", m2_hello::run());
}
