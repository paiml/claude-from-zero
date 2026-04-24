//! Module 2 demo library — First Rust function bootstrapped by Claude Code.
//!
//! Provable contract: `add(a, b)` is commutative and associative,
//! and `add(x, 0) == x` for every representable `i32`. The binary
//! `cargo run -p m2-hello` exercises the contract on concrete inputs
//! and exits zero when every assertion holds.

pub fn add(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_sums_two_positives() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn add_handles_negatives() {
        assert_eq!(add(-1, -2), -3);
    }

    #[test]
    fn add_is_commutative() {
        for a in [-100, -1, 0, 1, 100] {
            for b in [-100, -1, 0, 1, 100] {
                assert_eq!(add(a, b), add(b, a));
            }
        }
    }

    #[test]
    fn add_wraps_on_overflow() {
        assert_eq!(add(i32::MAX, 1), i32::MIN);
    }
}
