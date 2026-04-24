//! Provable contract: commutativity, associativity, additive identity.
//! A `cargo run` that exits zero is the proof.

use m2_hello::add;

fn main() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(3, 2), add(2, 3), "commutativity violated");
    assert_eq!(
        add(add(1, 2), 3),
        add(1, add(2, 3)),
        "associativity violated"
    );
    assert_eq!(add(42, 0), 42, "identity violated");
    assert_eq!(add(-7, 7), 0, "inverse violated");

    println!("contract: add is commutative, associative, has identity 0 — OK");
}
